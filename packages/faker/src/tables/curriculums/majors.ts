import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import { NANOID_LENGTH } from '../../generals.js'
import type { Curriculums, Majors } from '../../database.js'

export function generateMajors (curriculums: Array<Curriculums>, amountEach = 6): Array<Majors> {
  return curriculums.map(curriculum => {
    return Array.from({ length: amountEach }, (_, i) => {
      return {
        major_id: nanoid(NANOID_LENGTH),
        curriculum_id: curriculum.curriculum_id,
        major_name: faker.commerce.productName(),
        major_created_timestamp: faker.date.past(10).toISOString(),
        major_representative_id: i.toString().padStart(2, '0')
      }
    })
  }).flat()
}
