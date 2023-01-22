import { nanoid } from 'nanoid'
import { writeFile } from 'node:fs/promises'

import { faker } from '@faker-js/faker'

import type { Users } from '../../database.js'
import { ENCRYPTED_PASSWORD, NANOID_LENGTH, Role } from '../../generals.js'

export function generateAdmins (amount: number = 20): Array<Users> {
  return Array.from({ length: amount }, () => {
    const firstName = faker.name.firstName()
    const lastName = faker.name.lastName()

    const user: Users = {
      user_id: nanoid(NANOID_LENGTH),
      user_username: faker.internet.userName(firstName, lastName),
      user_email: faker.internet.email(firstName, lastName),
      user_password: ENCRYPTED_PASSWORD,
      user_role: Role.User,
      user_created_timestamp: faker.date.past(10).toISOString()
    }

    return user
  })
}
export async function saveAdmins (admins: Array<Users>): Promise<void> {
  await writeFile('../../../data/admins.json', JSON.stringify(admins))
}
