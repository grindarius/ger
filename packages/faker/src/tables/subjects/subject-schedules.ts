import dayjs from 'dayjs'
import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { Subjects, SubjectSchedules } from '../../database.js'
import { generateSubjectScheduleDays, NANOID_LENGTH } from '../../generals.js'

export function generateSubjectSchedules (subjects: Array<Subjects>, amountEach = 2): Array<SubjectSchedules> {
  return subjects.map(subject => {
    const subjectDays = generateSubjectScheduleDays()

    return Array.from({ length: amountEach }, (_, i) => {
      const startTimeNumber = faker.datatype.number({ min: 9, max: 16 })
      const startTime = dayjs().set('hour', startTimeNumber).set('minute', 0).set('second', 0).set('millisecond', 0).format('HH:mm:ss')
      const endTime = dayjs().set('hour', startTimeNumber + 2).set('minute', 0).set('second', 0).set('millisecond', 0).format('HH:mm:ss')

      return {
        subject_schedule_id: nanoid(NANOID_LENGTH),
        subject_id: subject.subject_id,
        subject_schedule_day_of_week: subjectDays[i],
        subject_schedule_start_time_of_day: startTime,
        subject_schedule_end_time_of_day: endTime
      }
    })
  }).flat()
}
