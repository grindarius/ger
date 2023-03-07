import dayjs from 'dayjs'
import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { ForumPostReplies, ForumPosts, Users } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateForumPostReplies (users: Array<Users>, posts: Array<ForumPosts>): [Array<ForumPosts>, Array<ForumPostReplies>] {
  const postReplies = posts.map(p => {
    const replyCreatedTimestamp = faker.date.between(p.forum_post_created_timestamp, new Date()).toISOString()

    return Array.from({ length: faker.datatype.number({ min: 2, max: 10 }) }, () => {
      const reply: ForumPostReplies = {
        forum_post_reply_id: nanoid(NANOID_LENGTH),
        forum_post_id: p.forum_post_id,
        user_id: faker.helpers.arrayElement(users).user_id,
        forum_post_reply_content: faker.lorem.paragraphs(3).replaceAll(',', ''),
        forum_post_reply_created_timestamp: replyCreatedTimestamp
      }

      return reply
    })
  }).flat()

  const postsWithUpdatedLastActiveTimestamp = posts.map(p => {
    const allRepliesInAPost = postReplies.filter(pr => {
      return pr.forum_post_id === p.forum_post_id
    })

    const latestPostTime = allRepliesInAPost.sort((a, b) => {
      return dayjs(b.forum_post_reply_created_timestamp).unix() - dayjs(a.forum_post_reply_created_timestamp).unix()
    })

    if (latestPostTime.length !== 0) {
      if (latestPostTime[0] == null) {
        return p
      }

      const postWithNewLastUpdatedTimestamp: ForumPosts = { ...p, forum_post_last_active_timestamp: latestPostTime[0].forum_post_reply_created_timestamp }
      return postWithNewLastUpdatedTimestamp
    }

    return p
  })

  return [postsWithUpdatedLastActiveTimestamp, postReplies]
}
