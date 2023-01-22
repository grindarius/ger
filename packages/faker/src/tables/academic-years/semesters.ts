import dayjs from 'dayjs'
import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { AcademicYears, Semesters } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateSemesters (academicYears: Array<AcademicYears>): Array<Semesters> {
  return academicYears.map(acadYear => {
    const howManySemesters = faker.datatype.number({
      min: 2,
      max: 4
    })

    const startDate = dayjs(acadYear.academic_year_start_timestamp)

    const semesters = Array.from({ length: howManySemesters }, () => {
      const semester: Semesters = {
        semester_id: nanoid(NANOID_LENGTH),
        academic_year_id: acadYear.academic_year_id,
        semester_start_timestamp: dayjs(startDate).endOf('month').toISOString(),
        semester_end_timestamp: dayjs(startDate).add(3, 'months').toISOString(),
        semester_created_timestamp: dayjs().toISOString()
      }

      startDate.endOf('month')
      return semester
    })

    return semesters
  }).flat()
}
