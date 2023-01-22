import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import { ENCRYPTED_PASSWORD, NANOID_LENGTH, Role } from '../../generals.js'
import type { Professors, Users } from '../../database.js'

export function generateProfessors (amount = 20): Array<[Users, Professors]> {
  return Array.from({ length: amount }, () => {
    const id = nanoid(NANOID_LENGTH)
    const firstName = faker.name.firstName()
    const lastName = faker.name.lastName()

    const user: Users = {
      user_id: id,
      user_username: faker.internet.userName(firstName, lastName),
      user_email: faker.internet.email(firstName, lastName),
      user_created_timestamp: faker.date.past(10).toISOString(),
      user_password: ENCRYPTED_PASSWORD,
      user_role: Role.Professor
    }

    const professor: Professors = {
      professor_id: id,
      professor_profile_image_path: '',
      professor_birthdate: faker.date.past(55).toISOString()
    }

    return [user, professor]
  })
}
