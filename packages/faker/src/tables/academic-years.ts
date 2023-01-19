import dayjs from 'dayjs'
import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

export interface AcademicYear {
  readonly academic_year_id: string
  readonly academic_year_gregorian_year: string
  readonly academic_year_start_timestamp: string
  readonly academic_year_end_timestamp: string
  readonly academic_year_created_timestamp: string
}

export function generateAcademicYears (startYear: number, endYear: number): Array<AcademicYear> {
  if (endYear <= startYear) {
    return []
  }

  return Array.from({ length: endYear - startYear }, (_, i) => {
    const academicYear: AcademicYear = {
      academic_year_id: nanoid(),
      academic_year_gregorian_year: (startYear + i).toString(),
      academic_year_start_timestamp: faker.date.between(dayjs(`${startYear + i}-06-01`).toDate(), dayjs(`${startYear + i}-07-01`).toDate()).toISOString(),
      academic_year_end_timestamp: faker.date.between(dayjs(`${startYear + i + 1}-02-01`).toDate(), dayjs(`${startYear + i + 1}-03-01`).toDate()).toISOString(),
      academic_year_created_timestamp: dayjs().toISOString()
    }

    return academicYear
  })
}
