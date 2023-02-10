import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import { type Users, Role } from '../../database.js'
import { ENCRYPTED_PASSWORD, NANOID_LENGTH } from '../../generals.js'

export function generateUsers (amount: number = 20): Array<Users> {
  return Array.from({ length: amount }, () => {
    const firstName = faker.name.firstName()
    const lastName = faker.name.lastName()

    const user: Users = {
      user_id: nanoid(NANOID_LENGTH),
      user_username: faker.internet.userName(firstName, lastName),
      user_email: faker.internet.email(firstName, lastName),
      user_password: ENCRYPTED_PASSWORD,
      user_role: Role.Admin,
      user_created_timestamp: faker.date.past(10).toISOString(),
      user_image_profile_path: '',
      user_birthdate: faker.date.birthdate().toISOString()
    }

    return user
  })
}
