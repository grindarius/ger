import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { Professors, Users } from '../../database.js'
import { Role } from '../../database.js'
import { ENCRYPTED_PASSWORD, NANOID_LENGTH } from '../../generals.js'

export function generateProfessors (amount = 20): Array<[Users, Professors]> {
  return Array.from({ length: amount }, () => {
    const id = nanoid(NANOID_LENGTH)
    const firstName = faker.name.firstName()
    const lastName = faker.name.lastName()

    const user: Users = {
      user_id: id,
      user_username: faker.internet.userName(firstName, lastName),
      user_email: faker.internet.email(firstName, lastName),
      user_password: ENCRYPTED_PASSWORD,
      user_role: Role.Professor,
      user_created_timestamp: faker.date.past(10).toISOString(),
      user_image_profile_path: '',
      user_birthdate: faker.date.birthdate().toISOString()
    }

    const professor: Professors = {
      professor_id: id,
      professor_professions: ''
    }

    return [user, professor]
  })
}
