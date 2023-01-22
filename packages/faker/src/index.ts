import dayjs from 'dayjs'
import timezone from 'dayjs/plugin/timezone.js'
import utc from 'dayjs/plugin/utc.js'
import { config } from 'dotenv-flow'

import { generateAcademicYears } from './tables/academic-years/academic-years.js'
import { generateCurriculums } from './tables/curriculums/curriculums.js'
import { generateFaculties } from './tables/curriculums/faculties.js'
import { generateMajors } from './tables/curriculums/majors.js'
import { generateAdmins } from './tables/users/admins.js'
import { generateProfessors } from './tables/users/professors.js'
import { generateStudents } from './tables/users/students.js'

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

const faculties = generateFaculties()
const curriculums = generateCurriculums(faculties)
const majors = generateMajors(curriculums)
const academicYears = generateAcademicYears(2000, 2022)

const admins = generateAdmins()
const professors = generateProfessors()
const students = generateStudents(majors, academicYears, professors[1])
