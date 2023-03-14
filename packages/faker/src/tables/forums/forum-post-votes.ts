import dayjs from 'dayjs'

import { faker } from '@faker-js/faker'

import type { ForumPosts, ForumPostViews, ForumPostVotes } from '../../database.js'

export function generateForumPostVotes (
  posts: Array<ForumPosts>,
  views: Array<ForumPostViews>
): Array<ForumPostVotes> {
  const postVotes = posts.map(p => {
    const usersWhoViewedThePost = views.filter(v => v.forum_post_id === p.forum_post_id)
    const sampledUsers = faker.helpers.arrayElements(usersWhoViewedThePost, faker.datatype.number({ min: 2, max: 5 }))

    let deactivatedDate = dayjs().toISOString()
    if (!p.forum_post_is_active) {
      deactivatedDate = p.forum_post_deactivated_timestamp ?? dayjs().toISOString()
    }

    return sampledUsers.map(u => {
      const pv: ForumPostVotes = {
        forum_post_id: p.forum_post_id,
        user_id: u.user_id,
        forum_post_vote_created_timestamp: faker.date.between(p.forum_post_created_timestamp, deactivatedDate).toISOString(),
        forum_post_vote_increment: faker.datatype.boolean() ? 1 : -1
      }

      return pv
    })
  }).flat()

  return postVotes
}
