import { config } from 'dotenv-flow'
import { writeFile } from 'node:fs/promises'

import { generateAcademicYears } from './tables/academic-years.js'
import { generateSemesters } from './tables/semesters.js'

config({
  path: '../../../.env.local'
})

// const username = process.env['GER_POSTGRES_USERNAME'] ?? ''
// const password = process.env['GER_POSTGRES_PASSWORD'] ?? ''
// const host = process.env['GER_POSTGRES_HOST'] ?? ''
// const port = process.env['GER_POSTGRES_PORT'] != null ? Number(process.env['GER_POSTGRES_PORT']) : 5432
// const databaseName = process.env['GER_POSTGRES_DATABASE_NAME'] ?? ''
//
// const pool = new Pool({
//   user: username,
//   password,
//   host,
//   port,
//   database: databaseName,
//   max: 20,
//   idleTimeoutMillis: 10000
// })

const academicYears = generateAcademicYears(2015, 2030)
const semesters = generateSemesters(academicYears)

await writeFile('../academic-years.json', JSON.stringify(academicYears)).catch(err => {
  console.error(err)
})
await writeFile('../semesters.json', JSON.stringify(semesters)).catch(err => {
  console.error(err)
})
