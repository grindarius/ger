import { nanoid } from 'nanoid'
import { writeFile } from 'node:fs/promises'

import { faker } from '@faker-js/faker'

import type { Faculties } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateFaculties (amount = 6): Array<Faculties> {
  return Array.from({ length: amount }, () => {
    return {
      faculty_id: nanoid(NANOID_LENGTH),
      faculty_name: faker.commerce.productName(),
      faculty_created_timestamp: faker.date.past(10).toISOString()
    }
  })
}
export async function saveFaculties (faculties: Array<Faculties>): Promise<void> {
  await writeFile('../../../data/faculties.json', JSON.stringify(faculties))
}
