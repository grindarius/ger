import { nanoid } from 'nanoid'

import type { GradingCriteriaGrades, GradingCriterias } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

const basicCriteria: Array<[string, number]> = [
  ['F', 0],
  ['D', 40],
  ['D+', 45],
  ['C', 60],
  ['C+', 65],
  ['B', 70],
  ['B+', 75],
  ['A', 80]
]

export function generateGradingCriteriaGrades (gradingCriterias: Array<GradingCriterias>): Array<GradingCriteriaGrades> {
  return gradingCriterias.map(criteria => {
    return Array.from({ length: 8 }, (_, i) => {
      return {
        grading_criteria_grade_id: nanoid(NANOID_LENGTH),
        grading_criteria_id: criteria.grading_criteria_id,
        grading_criteria_grade_alphabet: basicCriteria?.[i]?.[0] ?? '',
        grading_criteria_grade_minimum_score: basicCriteria?.[i]?.[1] ?? 0
      }
    })
  }).flat()
}
