package main

import (
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/gobeam/stringy"
	pg_query "github.com/pganalyze/pg_query_go/v2"
)

type Column struct {
	columnName string
	columnType *pg_query.Node
}

func convertColumnType(kind *pg_query.Node) (string, bool) {
	switch kind.GetString_().GetStr() {
	case "text":
		return "String", false
	case "float4":
		return "f64", false
	case "int4":
		return "i32", false
	case "int2":
		return "i16", false
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

				enumOutput += fmt.Sprintf("#[derive(ger_from_row::FromRow)]\npub enum %s {\n", enumNameCamelCase)
				typescriptEnumOutput += fmt.Sprintf("export enum %s {\n", enumNameCamelCase)
			} else if typeName[0].GetString_().GetStr() == "t_user_role" {
				enumName := stringy.New(typeName[0].GetString_().GetStr()[len(typeName[0].GetString_().GetStr())-4:])
				enumNameCamelCase := enumName.CamelCase()

				enumOutput += fmt.Sprintf("#[derive(ger_from_row::FromRow)]\npub enum %s {\n", enumNameCamelCase)
				typescriptEnumOutput += fmt.Sprintf("export enum %s {\n", enumNameCamelCase)
			}

			for _, enumValue := range statement.GetStmt().GetCreateEnumStmt().GetVals() {
				enumVariant := stringy.New(enumValue.GetString_().GetStr())
				enumVariantCamelCase := enumVariant.CamelCase()

				enumOutput += fmt.Sprintf("    %s,\n", enumVariantCamelCase)
				typescriptEnumOutput += fmt.Sprintf("  %s,\n", enumVariantCamelCase)
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
					columnNames = append(columnNames, Column{
						columnType: tableElement.GetColumnDef().GetTypeName().Names[len(tableElement.GetColumnDef().GetTypeName().Names)-1],
						columnName: tableElement.GetColumnDef().Colname,
					})
				}
			}

			tables[relationNameCamel] = columnNames
		}
	}

	output := ""
	typescriptOutput := ""

	for name, columns := range tables {
		output += fmt.Sprintf("#[derive(ger_from_row::FromRow)]\npub struct %s {\n", name)
		typescriptOutput += fmt.Sprintf("export interface %s {\n", name)

		for _, col := range columns {
			newType, isEnum := convertColumnType(col.columnType)
			newTypescriptType, _ := convertColumnTypeTypescript(col.columnType)

			typescriptOutput += fmt.Sprintf("  %s: %s\n", col.columnName, newTypescriptType)

			if isEnum {
				output += fmt.Sprintf("    #[fromrow(num)]\n    pub %s: %s,\n", col.columnName, newType)
				continue
			}

			output += fmt.Sprintf("    pub %s: %s,\n", col.columnName, newType)
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
