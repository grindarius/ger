import dayjs from 'dayjs'
import timezone from 'dayjs/plugin/timezone.js'
import utc from 'dayjs/plugin/utc.js'
import { config } from 'dotenv-flow'
import { writeFile } from 'node:fs/promises'

import { generateAcademicYears } from './tables/academic-years.js'
import { generateBuildings } from './tables/buildings.js'
import { generateRooms } from './tables/rooms.js'
import { generateSemesters } from './tables/semesters.js'

config({
  path: '../../../.env.local'
})

dayjs.extend(utc)
dayjs.extend(timezone)

dayjs.tz.setDefault('Asia/Bangkok')
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

const buildings = generateBuildings()
const rooms = generateRooms(buildings)

await writeFile('./data/academic-years.json', JSON.stringify(academicYears, null, 2)).catch(err => {
  console.error(err)
})
await writeFile('./data/semesters.json', JSON.stringify(semesters, null, 2)).catch(err => {
  console.error(err)
})

await writeFile('./data/buildings.json', JSON.stringify(buildings, null, 2)).catch(err => {
  console.error(err)
})
await writeFile('./data/rooms.json', JSON.stringify(rooms, null, 2)).catch(err => {
  console.error(err)
})
