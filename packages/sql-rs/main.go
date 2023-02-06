package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strings"

	"github.com/gobeam/stringy"
	pg_query "github.com/pganalyze/pg_query_go/v2"
)

type Column struct {
	columnName string
	columnType *pg_query.Node
	isNonNull  bool
}

func convertColumnType(kind *pg_query.Node) (string, bool) {
	switch kind.GetString_().GetStr() {
	case "text":
		return "String", false
	case "float4":
		return "f64", false
	case "numeric":
		return "f64", false
	case "int4":
		return "i32", false
	case "int2":
		return "i16", false
	case "date":
		return "time::Date", false
	case "timestamptz":
		return "time::OffsetDateTime", false
	case "time":
		return "time::Time", false
	case "point":
		return "geo_types::Point<f64>", false
	case "boolean":
		return "bool", false
	case "bool":
		return "bool", false
	case "t_user_role":
		return "Role", true
	case "t_day_of_week":
		return "DayOfWeek", true
	default:
		return "what", true
	}
}

func convertColumnTypeTypescript(kind *pg_query.Node) (string, bool) {
	switch kind.GetString_().GetStr() {
	case "text":
		return "string", false
	case "float4":
		return "number", false
	case "int4":
		return "number", false
	case "int2":
		return "number", false
	case "numeric":
		return "number", false
	case "date":
		return "string", false
	case "timestamptz":
		return "string", false
	case "time":
		return "string", false
	case "point":
		return "Point", false
	case "boolean":
		return "boolean", false
	case "bool":
		return "boolean", false
	case "t_user_role":
		return "Role", false
	case "t_day_of_week":
		return "DayOfWeek", false
	default:
		return "what", false
	}
}

// https://stackoverflow.com/a/70802740/12386405
func Contains(s []*pg_query.Node) bool {
	for _, v := range s {
		if v.GetConstraint().Contype == pg_query.ConstrType_CONSTR_NOTNULL {
			return true
		}
	}

	return false
}

func main() {
	file, err := os.ReadFile("../backend/database.sql")
	if err != nil {
		log.Fatal(err)
	}

	log.Println("file successfully loaded")

	tree, err := pg_query.Parse(string(file))
	if err != nil {
		log.Fatal(err)
	}

	log.Print("sql to ast parsed successfully")

	tables := make(map[string][]Column)

	enumOutput := ""
	typescriptEnumOutput := "export interface Point {\n  x: number\n  y: number\n}\n\n"

	for _, statement := range tree.Stmts {
		if statement.GetStmt().GetCreateEnumStmt() != nil {
			typeName := statement.GetStmt().GetCreateEnumStmt().GetTypeName()

			if typeName[0].GetString_().GetStr() == "t_day_of_week" {
				enumName := stringy.New(typeName[0].GetString_().GetStr()[2:])
				enumNameCamelCase := enumName.CamelCase()

				enumOutput += fmt.Sprintf("#[derive(\n    Debug,\n    PartialEq,\n    ger_from_row::FromRow,\n    serde::Serialize,\n    serde::Deserialize,\n    postgres_types::FromSql,\n    postgres_types::ToSql\n)]\n#[serde(rename_all = \"lowercase\")]\n#[postgres(name = \"%s\")]\npub enum %s {\n", typeName[0].GetString_().GetStr(), enumNameCamelCase)
				typescriptEnumOutput += fmt.Sprintf("export enum %s {\n", enumNameCamelCase)
			} else if typeName[0].GetString_().GetStr() == "t_user_role" {
				enumName := stringy.New(typeName[0].GetString_().GetStr()[len(typeName[0].GetString_().GetStr())-4:])
				enumNameCamelCase := enumName.CamelCase()

				enumOutput += fmt.Sprintf("#[derive(\n    Debug,\n    PartialEq,\n    ger_from_row::FromRow,\n    serde::Serialize,\n    serde::Deserialize,\n    postgres_types::FromSql,\n    postgres_types::ToSql\n)]\n#[serde(rename_all = \"lowercase\")]\n#[postgres(name = \"%s\")]\npub enum %s {\n", typeName[0].GetString_().GetStr(), enumNameCamelCase)
				typescriptEnumOutput += fmt.Sprintf("export enum %s {\n", enumNameCamelCase)
			}

			for _, enumValue := range statement.GetStmt().GetCreateEnumStmt().GetVals() {
				enumVariant := stringy.New(enumValue.GetString_().GetStr())
				enumVariantCamelCase := enumVariant.CamelCase()

				enumOutput += fmt.Sprintf("    #[postgres(name = \"%s\")]\n    %s,\n", enumVariant.ToLower(), enumVariantCamelCase)
				typescriptEnumOutput += fmt.Sprintf("  %s = \"%s\",\n", enumVariantCamelCase, enumVariant.ToLower())
			}

			enumOutput += "}\n\n"
			typescriptEnumOutput += "}\n\n"
		}

		if statement.GetStmt().GetCreateStmt().GetRelation() != nil {
			// used as struct name
			relationName := stringy.New(statement.GetStmt().GetCreateStmt().GetRelation().Relname)
			relationNameCamel := relationName.CamelCase()
			columnNames := []Column{}

			for _, tableElement := range statement.GetStmt().GetCreateStmt().GetTableElts() {
				if tableElement.GetColumnDef() != nil {
					isNonNull := Contains(tableElement.GetColumnDef().GetConstraints())

					columnNames = append(columnNames, Column{
						columnType: tableElement.GetColumnDef().GetTypeName().Names[len(tableElement.GetColumnDef().GetTypeName().Names)-1],
						columnName: tableElement.GetColumnDef().Colname,
						isNonNull:  isNonNull,
					})
				}
			}

			tables[relationNameCamel] = columnNames
		}
	}

	output := ""
	typescriptOutput := ""

	tableKeys := make([]string, 0, len(tables))

	for k := range tables {
		tableKeys = append(tableKeys, k)
	}

	sort.Strings(tableKeys)

	for _, name := range tableKeys {
		output += fmt.Sprintf("#[derive(ger_from_row::FromRow, serde::Serialize, serde::Deserialize)]\npub struct %s {\n", name)
		typescriptOutput += fmt.Sprintf("export interface %s {\n", name)

		for _, col := range tables[name] {
			newType, isEnum := convertColumnType(col.columnType)
			newTypescriptType, _ := convertColumnTypeTypescript(col.columnType)

			if col.isNonNull {
				typescriptOutput += fmt.Sprintf("  %s: %s\n", col.columnName, newTypescriptType)
			} else {
				typescriptOutput += fmt.Sprintf("  %s?: %s\n", col.columnName, newTypescriptType)
			}

			if isEnum {
				output += fmt.Sprintf("    #[fromrow(num)]\n")
			}

			if col.isNonNull {
				output += fmt.Sprintf("    pub %s: %s,\n", col.columnName, newType)
			} else {
				output += fmt.Sprintf("    pub %s: Option<%s>,\n", col.columnName, newType)
			}
		}

		output += "}\n\n"
		typescriptOutput += "}\n\n"
	}

	output = strings.TrimSuffix(output, "\n")
	typescriptOutput = strings.TrimSuffix(typescriptOutput, "\n")

	err = os.WriteFile("../backend/src/database.rs", []byte(enumOutput+output), 0666)

	if err != nil {
		log.Fatal(err)
	}

	err = os.WriteFile("../faker/src/database.ts", []byte(typescriptEnumOutput+typescriptOutput), 0666)

	if err != nil {
		log.Fatal(err)
	}

	log.Println("file saved successfully")
}
