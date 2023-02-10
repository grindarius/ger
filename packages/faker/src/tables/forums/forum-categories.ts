import { nanoid } from 'nanoid'
import slug from 'slug'

import { faker } from '@faker-js/faker'

import type { ForumCategories, Users } from '../../database.js'
import { Role } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateForumCategories (users: Array<Users>): Array<ForumCategories> {
  const admins = users.filter(u => u.user_role === Role.Admin)

  return [
    {
      forum_category_id: nanoid(NANOID_LENGTH),
      forum_category_name: 'Help',
      forum_category_representative_id: slug('Help'),
      user_id: faker.helpers.arrayElement(admins).user_id,
      forum_category_color_theme: faker.color.rgb({ prefix: '#' }),
      forum_category_created_timestamp: faker.date.past(10).toISOString()
    },
    {
      forum_category_id: nanoid(NANOID_LENGTH),
      forum_category_name: 'Advertisements',
      forum_category_representative_id: 'advertisements',
      user_id: faker.helpers.arrayElement(admins).user_id,
      forum_category_color_theme: faker.color.rgb({ prefix: '#' }),
      forum_category_created_timestamp: faker.date.past(10).toISOString()
    },
    {
      forum_category_id: nanoid(NANOID_LENGTH),
      forum_category_name: 'Uncategorized',
      forum_category_representative_id: 'uncategorized',
      user_id: faker.helpers.arrayElement(admins).user_id,
      forum_category_color_theme: faker.color.rgb({ prefix: '#' }),
      forum_category_created_timestamp: faker.date.past(10).toISOString()
    },
    {
      forum_category_id: nanoid(NANOID_LENGTH),
      forum_category_name: 'Memes',
      forum_category_representative_id: 'memes',
      user_id: faker.helpers.arrayElement(admins).user_id,
      forum_category_color_theme: faker.color.rgb({ prefix: '#' }),
      forum_category_created_timestamp: faker.date.past(10).toISOString()
    },
    {
      forum_category_id: nanoid(NANOID_LENGTH),
      forum_category_name: 'Homeworks',
      forum_category_representative_id: slug('Homeworks'),
      user_id: faker.helpers.arrayElement(admins).user_id,
      forum_category_color_theme: faker.color.rgb({ prefix: '#' }),
      forum_category_created_timestamp: faker.date.past(10).toISOString()
    }
  ]
}
