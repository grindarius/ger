import { faker } from '@faker-js/faker'

import type { ForumPostReplies, ForumPostReplyVotes, Users } from '../../database.js'

export function generateForumPostReplyVotes (users: Array<Users>, replies: Array<ForumPostReplies>): Array<ForumPostReplyVotes> {
  return replies.map(r => {
    const sampledUsers = faker.helpers.arrayElements(users, faker.datatype.number({ min: 2, max: 5 }))

    return sampledUsers.map(u => {
      const vote: ForumPostReplyVotes = {
        forum_post_reply_id: r.forum_post_reply_id,
        user_id: u.user_id,
        forum_post_reply_vote_created_timestamp: faker.date.between(r.forum_post_reply_created_timestamp, new Date()).toISOString(),
        forum_post_reply_vote_increment: faker.datatype.boolean() ? 1 : -1
      }

      return vote
    })
  }).flat()
}
