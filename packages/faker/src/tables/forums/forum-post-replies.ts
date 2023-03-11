import dayjs from 'dayjs'
import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { ForumPostReplies, ForumPosts, Users } from '../../database.js'
import { generateShortBlockquote, NANOID_LENGTH } from '../../generals.js'

export function generateForumPostReplies (users: Array<Users>, posts: Array<ForumPosts>): [Array<ForumPosts>, Array<ForumPostReplies>] {
  const postReplies = posts.map(p => {
    const replyCreatedTimestamp = faker.date.between(p.forum_post_created_timestamp, new Date()).toISOString()

    return Array.from({ length: faker.datatype.number({ min: 2, max: 10 }) }, () => {
      const reply: ForumPostReplies = {
        forum_post_reply_id: nanoid(NANOID_LENGTH),
        forum_post_id: p.forum_post_id,
        user_id: faker.helpers.arrayElement(users).user_id,
        forum_post_reply_content: generateShortBlockquote(),
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

    // If the post does not have any comments, return normal `post`
    if (latestPostTime[0] == null) {
      return p
    }

    // update the post with new latest updated time
    let updatedPost: ForumPosts = { ...p, forum_post_last_active_timestamp: latestPostTime[0].forum_post_reply_created_timestamp }

    // if the post is locked, update locked timestamp with latest post.
    if (!p.forum_post_is_active) {
      updatedPost = {
        ...updatedPost,
        forum_post_deactivated_timestamp: dayjs(latestPostTime[0].forum_post_reply_created_timestamp)
          .add(faker.datatype.number({ min: 1, max: 10 }), 'minutes')
          .toISOString()
      }
    }

    return updatedPost
  })

  return [postsWithUpdatedLastActiveTimestamp, postReplies]
}
