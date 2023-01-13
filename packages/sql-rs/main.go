package main

import (
	"fmt"
	"github.com/gobeam/stringy"
	"github.com/pganalyze/pg_query_go/v2"
	"log"
	"os"
)

type Column struct {
	columnName string
	columnType *pg_query.Node
}

func convertColumnType(kind *pg_query.Node) string {
	switch kind.GetString_().GetStr() {
	case "text":
		return "String"
	case "float4":
		return "f64"
	case "int4":
		return "i32"
	case "timestamptz":
		return "time::OffsetDateTime"
	case "time":
		return "time::Time"
	case "point":
		return "geo_types::Point<f64>"
	case "t_role":
		return "Role"
	case "t_day_of_week":
		return "DayOfWeek"
	default:
		return "what"
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

				enumOutput += fmt.Sprintf("enum %s {\n", enumNameCamelCase)
			} else if typeName[0].GetString_().GetStr() == "t_user_role" {
				enumName := stringy.New(typeName[0].GetString_().GetStr()[len(typeName[0].GetString_().GetStr())-4:])
				enumNameCamelCase := enumName.CamelCase()

				enumOutput += fmt.Sprintf("pub enum %s {\n", enumNameCamelCase)
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
		output += fmt.Sprintf("pub struct %s {\n", name)

		for _, col := range columns {
			output += fmt.Sprintf("    pub %s: %s,\n", col.columnName, convertColumnType(col.columnType))
		}

		output += "}\n\n"
	}

	writeFileErr := os.WriteFile("../backend/src/database.rs", []byte(enumOutput+output), 0666)

	if writeFileErr != nil {
		log.Fatal(writeFileErr)
	}

	log.Println("file saved successfully")
}
