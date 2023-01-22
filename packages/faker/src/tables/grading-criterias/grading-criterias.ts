import { nanoid } from 'nanoid'
import { writeFile } from 'node:fs/promises'

import { faker } from '@faker-js/faker'

import type { GradingCriterias, Users } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export interface GradingCriteria {
  grading_criteria_id: string
  user_id: string
  grading_criteria_name: string
  grading_criteria_created_timestamp: string
}

export function generateGradingCriterias (users: Array<Users>): Array<GradingCriteria> {
  return [{
    grading_criteria_id: nanoid(NANOID_LENGTH),
    user_id: faker.helpers.arrayElement(users).user_id,
    grading_criteria_name: faker.commerce.productName(),
    grading_criteria_created_timestamp: faker.date.past(2).toISOString()
  }]
}

export async function saveGradingCriterias (gradingCriterias: Array<GradingCriterias>): Promise<void> {
  await writeFile('../../../data/grading-criterias.json', JSON.stringify(gradingCriterias))
}
