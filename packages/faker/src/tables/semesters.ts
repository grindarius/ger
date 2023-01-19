import dayjs from 'dayjs'
import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { AcademicYear } from './academic-years.js'

export interface Semester {
  readonly semester_id: string
  readonly academic_year_id: string
  readonly semester_start_timestamp: string
  readonly semester_end_timestamp: string
  readonly semester_created_timestamp: string
}

export function generateSemesters (academicYears: Array<AcademicYear>): Array<Semester> {
  return academicYears.map(acadYear => {
    const howManySemesters = faker.datatype.number({
      min: 2,
      max: 4
    })

    const startDate = dayjs(acadYear.academic_year_start_timestamp)

    const semesters = Array.from({ length: howManySemesters }, () => {
      const semester: Semester = {
        semester_id: nanoid(),
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
