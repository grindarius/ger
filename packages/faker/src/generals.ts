import { customAlphabet } from 'nanoid'
import { writeFile } from 'node:fs/promises'
import { resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import type pg from 'pg'

import { faker } from '@faker-js/faker'

import { DayOfWeek } from './database.js'
import { logger } from './index.js'

/* eslint-disable-next-line @typescript-eslint/naming-convention */
const __dirname = fileURLToPath(new URL('.', import.meta.url))

export const NANOID_LENGTH = 32

export interface Point {
  readonly x: number
  readonly y: number
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

function calculateNIDChecksum (first12digits: string): number {
  const first12digitsArray = first12digits.split('')

  const checksum = first12digitsArray.reduce((previous, current, i) => {
    return (Number(current) * (13 - i)) + previous
  }, 0)

  const moddedChecksum = checksum % 11
  return 11 - moddedChecksum
}

export function nid (): string {
  const first12digits = customAlphabet('0123456789', 12)()
  const checksum = calculateNIDChecksum(first12digits)
  return first12digits + checksum.toString()
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

export function generateMarkdownContent (): string {
  const headers = Array.from({ length: 6 }, (_, i) => `${' '.padStart(i + 1, '#')} H${i + 1}` + '\n')
  const emphasises = `*italics* and _italics_
    **strong emphasis** and __strong emphasis__
    **combined _emphasis_**
    ~~strikethrough~~
    `

  const lists = Array.from({ length: 3 }, (_, i) => faker.helpers.fake(`${i + 1}. {{commerce.productName}}`) + '\n')
  const links = `[inline link](https://google.com)
    [inline with title](https://google.com "Google's Homepage")
    [reference link](reddit)
    or use [twitter]

    [reddit]: https://reddit.com
    [twitter]: https://twitter.com
    `
  const inlineCode = 'inline `code` with `backticks`'
  const codeLang = '```javascript\n' +
    'function add(one, other) {\n' +
    '  return one + other\n' +
    '}\n' +
    '```\n\n' +
    '```\n' +
    '$ exec "$SHELL"\n' +
    '```\n\n'

  const images = '![inline image](https://imgur.com/HFD0Sy0 "look ka tis")'

  const footnotes = 'simple footnotes[^1]\n\n\n[^1]: mi refrences.'

  const tables = `
| Tables        | Are           | Cool  |
| ------------- |:-------------:| -----:|
| col 3 is      | right-aligned | $1600 |
| col 2 is      | centered      |   $12 |
| zebra stripes | are neat      |    $1 |
`

  const blockquotes = '> quotes this text pls\n\n' +
    'break\n\n' +
    '> another quotes\n\n'

  const inlineHTML = `
<dl>
  <dt>Definition list</dt>
  <dd> lollllll</dd>

  <dt>Definition list</dt>
  <dd> lollllll</dd>
</dl>
`

  const rules = '---\n\n' +
    '***\n\n' +
    '___\n\n'

  return [
    headers,
    emphasises,
    lists,
    links,
    inlineCode,
    codeLang,
    images,
    footnotes,
    tables,
    blockquotes,
    inlineHTML,
    rules
  ].join('\n')
}

export function generateShortBlockquote (): string {
  const blockquote = faker.helpers.fake('> {{lorem.paragraph(20)}}')
  const normalReply = faker.lorem.paragraph(10)

  return [blockquote, normalReply].join('\n\n')
}
