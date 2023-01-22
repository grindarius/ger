import { nanoid } from 'nanoid'
import { writeFile } from 'node:fs/promises'

import { faker } from '@faker-js/faker'

import type { Subjects } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export interface Subject {
  subject_id: string
  subject_name: string
  subject_credit: number
  subject_description: string
  subject_created_timestamp: string
}

export function generateSubjects (amount = 50): Array<Subject> {
  return Array.from({ length: amount }, () => {
    return {
      subject_id: nanoid(NANOID_LENGTH),
      subject_name: faker.commerce.productName(),
      subject_credit: faker.datatype.number({ min: 1, max: 6 }),
      subject_description: faker.commerce.productDescription(),
      subject_created_timestamp: faker.date.past(8).toISOString()
    }
  })
}

export async function saveSubjects (subjects: Array<Subjects>): Promise<void> {
  await writeFile('../../../data/subjects.json', JSON.stringify(subjects))
}
