import { parse } from 'pgsql-parser'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { readFile, writeFile } from 'node:fs/promises'

const __dirname = dirname(fileURLToPath(import.meta.url))

const fileString = await readFile(join(__dirname, '..', '..', 'ger', 'database.sql'), { encoding: 'utf-8' })

const tables = parse(fileString)

console.log(tables)

await writeFile(join(__dirname, '..', 'cst.json'), JSON.stringify(tables, null, 4))
