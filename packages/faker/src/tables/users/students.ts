import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { AcademicYears, Majors, Professors, Students, Users } from '../../database.js'
import { ENCRYPTED_PASSWORD, NANOID_LENGTH, nid, Role } from '../../generals.js'

export function generateStudents (
  majors: Array<Majors>,
  academicYears: Array<AcademicYears>,
  professors: Array<Professors>,
  amount = 200
): Array<[Users, Students]> {
  return Array.from({ length: amount }, (_, i) => {
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
    const firstAcadYearAD = Number(firstAcadYear.academic_year_gregorian_year) + 543

    const student: Students = {
      student_id: id,
      student_representative_id: `${firstAcadYearAD.toString().substring(2)}${faker.helpers.arrayElement(majors).major_representative_id}${i.toString().padStart(4, '0')}`,
      student_profile_image_path: '',
      student_nid: nid(),
      student_birthdate: faker.date.past(30).toISOString(),
      student_previous_school_name: faker.company.name(),
      student_previous_school_gpa: faker.datatype.number({ min: 2, max: 4, precision: 0.01 }),
      major_id: nanoid(NANOID_LENGTH),
      professor_id: faker.helpers.arrayElement(professors).professor_id,
      first_academic_year_id: firstAcadYear.academic_year_id
    }

    return [user, student]
  })
}
