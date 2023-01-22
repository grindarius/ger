import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import { NANOID_LENGTH } from '../../generals.js'
import type { MajorCreditSpecifications, Majors } from '../../database.js'

export function generateMajorCreditSpecifications (majors: Array<Majors>, amountEach: 5): Array<MajorCreditSpecifications> {
  return majors.map(major => {
    return Array.from({ length: amountEach }, () => {
      return {
        major_credit_specification_id: nanoid(NANOID_LENGTH),
        major_id: major.major_id,
        major_credit_specification_name: faker.commerce.productName(),
        major_credit_specification_minimum_credit: faker.datatype.number({ min: 2, max: 8 }) * 3,
        major_credit_specification_created_timestamp: faker.date.past(8).toISOString()
      }
    })
  }).flat()
}
