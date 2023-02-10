import { faker } from '@faker-js/faker'

import type { ForumPosts, ForumPostViews, Users } from '../../database.js'

export function generateForumPostViews (
  users: Array<Users>,
  posts: Array<ForumPosts>,
): Array<ForumPostViews> {
  const postViews = posts.map(p => {
    const sampledUsers = faker.helpers.arrayElements(users, faker.datatype.number({ min: 20, max: 50 }))

    return sampledUsers.map(u => {
      const view: ForumPostViews = {
        forum_post_id: p.forum_post_id,
        user_id: u.user_id
      }

      return view
    })
  }).flat()

  return postViews
}
