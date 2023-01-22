import { nanoid } from 'nanoid'
import { writeFile } from 'node:fs/promises'

import { faker } from '@faker-js/faker'

import type { Curriculums, Faculties } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateCurriculums (faculties: Array<Faculties>, amountEach = 6): Array<Curriculums> {
  return faculties.map(faculty => {
    return Array.from({ length: amountEach }, () => {
      return {
        curriculum_id: nanoid(NANOID_LENGTH),
        faculty_id: faculty.faculty_id,
        curriculum_name: faker.company.name(),
        curriculum_created_timestamp: faker.date.past(10).toISOString()
      }
    })
  }).flat()
}
export async function saveCurriculums (curriculums: Array<Curriculums>): Promise<void> {
  await writeFile('../../../data/curriculums.json', JSON.stringify(curriculums))
}
