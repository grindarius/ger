import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import { NANOID_LENGTH } from '../../generals.js'
import type { User } from '../users/admins.js'

export interface GradingCriteria {
  grading_criteria_id: string
  user_id: string
  grading_criteria_name: string
  grading_criteria_created_timestamp: string
}

export function generateGradingCriteria (users: Array<User>): Array<GradingCriteria> {
  return [{
    grading_criteria_id: nanoid(NANOID_LENGTH),
    user_id: faker.helpers.arrayElement(users).user_id,
    grading_criteria_name: faker.commerce.productName(),
    grading_criteria_created_timestamp: faker.date.past(2).toISOString()
  }]
}
