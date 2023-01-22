import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

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
