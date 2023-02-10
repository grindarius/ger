import dayjs from 'dayjs'
import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { ForumCategories, ForumPosts, Users } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateForumPosts (users: Array<Users>, categories: Array<ForumCategories>, amount = 200): Array<ForumPosts> {
  return Array.from({ length: amount }, () => {
    const createdTimestamp = faker.date.between(dayjs().subtract(8, 'years').toDate(), dayjs().subtract(2, 'years').toDate())
    const isActive = faker.datatype.boolean()
    const lastActiveDate = faker.date.between(createdTimestamp, new Date())

    const post: ForumPosts = {
      forum_post_id: nanoid(NANOID_LENGTH),
      forum_post_name: faker.commerce.productName(),
      user_id: faker.helpers.arrayElement(users).user_id,
      forum_category_id: faker.helpers.arrayElement(categories).forum_category_id,
      forum_post_content: faker.lorem.paragraph(100).replaceAll(',', ''),
      forum_post_is_active: isActive,
      forum_post_created_timestamp: createdTimestamp.toISOString(),
      forum_post_last_active_timestamp: lastActiveDate.toISOString(),
      forum_post_is_category_based_announcement: faker.datatype.boolean(),
      forum_post_deactivated_timestamp: !isActive ? faker.date.between(lastActiveDate, new Date()).toISOString() : null,
      forum_post_is_global_announcement: faker.datatype.boolean()
    }

    return post
  })
}
