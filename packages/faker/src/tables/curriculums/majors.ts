import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { Curriculums, Majors } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

function getMajorRepresentativeId (previousMajors: Array<Majors>): string {
  const latestMajorId = previousMajors.sort((a, b) => Number(b.major_representative_id) - Number(a.major_representative_id))[0]
  return ((Number(latestMajorId?.major_representative_id ?? 0) ?? 0) + 1).toString()
}

export function generateMajors (curriculums: Array<Curriculums>, amountEach = 3): Array<Majors> {
  const majors: Array<Majors> = []

  for (const curriculum of curriculums) {
    for (let i = 0; i < amountEach; i++) {
      const major: Majors = {
        major_id: nanoid(NANOID_LENGTH),
        curriculum_id: curriculum.curriculum_id,
        major_name: faker.commerce.productName(),
        major_created_timestamp: faker.date.past(10).toISOString(),
        major_representative_id: getMajorRepresentativeId(majors)
      }

      majors.push(major)
    }
  }

  return majors
}
