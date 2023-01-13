package main

import (
	"fmt"
	"log"
	"os"

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

	for _, statement := range tree.Stmts {
		if statement.GetStmt().GetCreateEnumStmt() != nil {
			typeName := statement.GetStmt().GetCreateEnumStmt().GetTypeName()

			if typeName[0].GetString_().GetStr() == "t_day_of_week" {
				enumName := stringy.New(typeName[0].GetString_().GetStr()[2:])
				enumNameCamelCase := enumName.CamelCase()

				enumOutput += fmt.Sprintf("#[derive(ger_from_row::FromRow)]\npub enum %s {\n", enumNameCamelCase)
			} else if typeName[0].GetString_().GetStr() == "t_user_role" {
				enumName := stringy.New(typeName[0].GetString_().GetStr()[len(typeName[0].GetString_().GetStr())-4:])
				enumNameCamelCase := enumName.CamelCase()

				enumOutput += fmt.Sprintf("#[derive(ger_from_row::FromRow)]\npub enum %s {\n", enumNameCamelCase)
			}

			for _, enumValue := range statement.GetStmt().GetCreateEnumStmt().GetVals() {
				enumVariant := stringy.New(enumValue.GetString_().GetStr())
				enumVariantCamelCase := enumVariant.CamelCase()

				enumOutput += fmt.Sprintf("    %s,\n", enumVariantCamelCase)
			}

			enumOutput += "}\n\n"
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

	for name, columns := range tables {
		output += fmt.Sprintf("#[derive(ger_from_row::FromRow)]\npub struct %s {\n", name)

		for _, col := range columns {
			newType, isEnum := convertColumnType(col.columnType)

			if isEnum {
				output += fmt.Sprintf("    #[fromrow(num)]\n    pub %s: %s,\n", col.columnName, newType)
				continue
			}

			output += fmt.Sprintf("    pub %s: %s,\n", col.columnName, newType)
		}

		output += "}\n\n"
	}

	writeFileErr := os.WriteFile("../backend/src/database.rs", []byte(enumOutput+output), 0666)

	if writeFileErr != nil {
		log.Fatal(writeFileErr)
	}

	log.Println("file saved successfully")
}
