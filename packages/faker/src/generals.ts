import { writeFile } from 'node:fs/promises'
import { resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import type pg from 'pg'

import { faker } from '@faker-js/faker'

import { logger } from './index.js'

/* eslint-disable-next-line @typescript-eslint/naming-convention */
const __dirname = fileURLToPath(new URL('.', import.meta.url))

export const NANOID_LENGTH = 32

export interface Point {
  readonly x: number
  readonly y: number
}

export enum Role {
  Admin = 'admin',
  Student = 'student',
  Professor = 'professor'
}

export enum DayOfWeek {
  Sunday = 'sunday',
  Monday = 'monday',
  Tuesday = 'tuesday',
  Wednesday = 'wednesday',
  Thursday = 'thursday',
  Friday = 'friday',
  Saturday = 'saturday'
}

const dayOfWeekArray = [
  'sunday',
  'monday',
  'tuesday',
  'wednesday',
  'thursday',
  'friday',
  'saturday'
]

function stringToDayOfWeek (days: Array<string>): Array<DayOfWeek> {
  return days.map(day => {
    if (day === 'sunday') {
      return DayOfWeek.Sunday
    }

    if (day === 'monday') {
      return DayOfWeek.Monday
    }

    if (day === 'tuesday') {
      return DayOfWeek.Tuesday
    }

    if (day === 'wednesday') {
      return DayOfWeek.Wednesday
    }

    if (day === 'thursday') {
      return DayOfWeek.Thursday
    }

    if (day === 'friday') {
      return DayOfWeek.Friday
    }

    if (day === 'saturday') {
      return DayOfWeek.Saturday
    }

    return DayOfWeek.Monday
  })
}

export function generateSubjectScheduleDays (): [DayOfWeek, DayOfWeek] {
  const days = faker.helpers.arrayElements(dayOfWeekArray, 2)

  return stringToDayOfWeek(days) as [DayOfWeek, DayOfWeek]
}

export function coordsArrayToPoint (coords: [string, string]): Point {
  const point: Point = {
    x: Number(coords[0]),
    y: Number(coords[1])
  }

  return point
}

function calculateNIDChecksum (first12digits: number): number {
  const first12digitsArray = first12digits.toString().split('')

  const checksum = first12digitsArray.reduce((previous, current, i) => {
    return (Number(current) * (13 - i)) + previous
  }, 0)

  const moddedChecksum = checksum % 11
  return 11 - moddedChecksum
}

export function nid (): string {
  const first12digits = faker.datatype.number({
    min: 100000000000,
    max: 899999999999
  })

  const checksum = calculateNIDChecksum(first12digits)
  return first12digits.toString() + checksum.toString()
}

export const UNENCRYPTED_PASSWORD = 'aryastark'
export const ENCRYPTED_PASSWORD = '$argon2id$v=19$m=4096,t=12,p=6$Yw4TlZTREJt9FX15Qbp5wcOaf0rs6z+MDYjgOx+i/vBfSCqel7DSa1UG$KKR3zYuM98AVnhADF920hB/cAFjByy/3maLCQH5lWGvguBCBAEIWgFZQv6rJcdNHOeB3Pw3Y20ZXqHyWMXTnQQ'

export async function saveToFile<T> (contents: Array<T>, filename: string): Promise<void> {
  await writeFile(resolve(__dirname, '..', 'data', filename), JSON.stringify(contents, null, 2))
}

export async function insert<T extends object> (contents: Array<T>, tableName: string, pool: pg.Pool): Promise<void> {
  const columns = Object.keys(contents[0] ?? {})
  const columnString = columns.join(', ')
  const columnValues = columns.map((_, i) => '$' + (i + 1).toString()).join(', ')

  for (const row of contents) {
    const rowData = Object.values(row)

    logger.info(`insert into "${tableName}" (${columnString}) values (${columnValues}) (${rowData.join(', ')})`)
    await pool.query(`insert into "${tableName}" (${columnString}) values (${columnValues})`, rowData)
  }
}
