import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { Curriculums, Majors } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateMajors (curriculums: Array<Curriculums>, amountEach = 3): Array<Majors> {
  return curriculums.map(curriculum => {
    const representativeIds: Array<string> = Array.from({ length: faker.datatype.number({ min: 1, max: 3 }) }, () => {
      return faker.datatype.number({ min: 1, max: 99 }).toString().padStart(2, '0')
    })

    return Array.from({ length: amountEach }, () => {
      return {
        major_id: nanoid(NANOID_LENGTH),
        curriculum_id: curriculum.curriculum_id,
        major_name: faker.commerce.productName(),
        major_created_timestamp: faker.date.past(10).toISOString(),
        major_representative_id: faker.helpers.arrayElement(representativeIds)
      }
    })
  }).flat()
}
