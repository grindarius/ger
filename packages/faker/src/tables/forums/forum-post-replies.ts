import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { ForumPostReplies, ForumPosts, Users } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateForumPostReplies (users: Array<Users>, posts: Array<ForumPosts>): Array<ForumPostReplies> {
  const postReplies = posts.map(p => {
    return Array.from({ length: faker.datatype.number({ min: 0, max: 10 }) }, () => {
      const reply: ForumPostReplies = {
        forum_post_reply_id: nanoid(NANOID_LENGTH),
        forum_post_id: p.forum_post_id,
        user_id: faker.helpers.arrayElement(users).user_id,
        forum_post_reply_content: faker.lorem.paragraphs(3).replaceAll(',', ''),
        forum_post_reply_created_timestamp: faker.date.between(p.forum_post_created_timestamp, new Date()).toISOString()
      }

      return reply
    })
  }).flat()

  return postReplies
}
