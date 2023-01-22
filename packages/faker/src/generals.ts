import { faker } from '@faker-js/faker'
import type { Options } from '@node-rs/argon2'
import { hash } from '@node-rs/argon2'

export const NANOID_LENGTH = 32

export interface Point {
  readonly x: number
  readonly y: number
}

export enum Role {
  User = 'user',
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

const hashOptions: Options = {
  parallelism: 6,
  secret: Buffer.from(process.env['GER_ARGON2_SALT'] ?? '')
}

export const UNENCRYPTED_PASSWORD = 'aryastark'
export const ENCRYPTED_PASSWORD = await hash(UNENCRYPTED_PASSWORD, hashOptions)
