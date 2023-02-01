import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { AcademicYears, Majors, Professors, Students, Users } from '../../database.js'
import { ENCRYPTED_PASSWORD, NANOID_LENGTH, nid, Role } from '../../generals.js'
// import { logger } from '../../index.js'

/// get latest index by major id and academic year id
function getRepresentativeId (previousStudents: Array<[Users, Students]>, major: Majors, firstAcadYear: AcademicYears): string {
  const firstAcadYearBE = Number(firstAcadYear.academic_year_anno_domini_year) + 543
  const studentsInSameMajorAndYear = previousStudents.filter(ps => ps[1].major_id === major.major_id && ps[1].first_academic_year_id === firstAcadYear.academic_year_id)
  const latestStudentId = studentsInSameMajorAndYear.sort((a, b) => {
    return Number(b[1].student_representative_id.slice(4)) - Number(a[1].student_representative_id.slice(4))
  })

  const template = `${firstAcadYearBE.toString().substring(2)}${major.major_representative_id}${((Number(latestStudentId?.[0]?.[1].student_representative_id.slice(4) ?? 0) ?? 0) + 1).toString().padStart(4, '0')}`

  if (previousStudents.map(p => p[1].student_representative_id).includes(template)) {
    const msg = `redundant student representative id found: representative_id: ${template}`
    throw new Error(msg)
  }

  return template
}

export function generateStudents (
  majors: Array<Majors>,
  academicYears: Array<AcademicYears>,
  professors: Array<Professors>,
  amount = 200
): Array<[Users, Students]> {
  const students: Array<[Users, Students]> = []

  for (let i = 0; i < amount; i++) {
    const id = nanoid(NANOID_LENGTH)
    const firstName = faker.name.firstName()
    const lastName = faker.name.lastName()

    const user: Users = {
      user_id: id,
      user_username: faker.internet.userName(firstName, lastName),
      user_email: faker.internet.email(firstName, lastName),
      user_password: ENCRYPTED_PASSWORD,
      user_role: Role.Student,
      user_created_timestamp: faker.date.past(18).toISOString()
    }

    const firstAcadYear = faker.helpers.arrayElement(academicYears)
    const major = faker.helpers.arrayElement(majors)

    const student: Students = {
      student_id: id,
      student_representative_id: getRepresentativeId(students, major, firstAcadYear),
      student_profile_image_path: '',
      student_nid: nid(),
      student_birthdate: faker.date.past(30).toISOString(),
      student_previous_school_name: faker.company.name(),
      student_previous_school_gpa: faker.datatype.number({ min: 2, max: 4, precision: 0.01 }),
      major_id: major.major_id,
      professor_id: faker.helpers.arrayElement(professors).professor_id,
      first_academic_year_id: firstAcadYear.academic_year_id
    }

    students.push([user, student])
  }

  return students
}
