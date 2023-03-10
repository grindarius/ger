import dayjs from 'dayjs'
import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { AcademicYears } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateAcademicYears (startYear: number, endYear: number): Array<AcademicYears> {
  if (endYear <= startYear) {
    return []
  }

  return Array.from({ length: endYear - startYear }, (_, i) => {
    const academicYear: AcademicYears = {
      academic_year_id: nanoid(NANOID_LENGTH),
      academic_year_anno_domini_year: (startYear + i).toString(),
      academic_year_start_timestamp: faker.date.between(dayjs(`${startYear + i}-06-01`).toDate(), dayjs(`${startYear + i}-07-01`).toDate()).toISOString(),
      academic_year_end_timestamp: faker.date.between(dayjs(`${startYear + i + 1}-02-01`).toDate(), dayjs(`${startYear + i + 1}-03-01`).toDate()).toISOString(),
      academic_year_created_timestamp: dayjs().toISOString()
    }

    return academicYear
  })
}
