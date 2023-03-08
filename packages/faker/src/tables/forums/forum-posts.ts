import dayjs from 'dayjs'
import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { ForumCategories, ForumPosts, Users } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateForumPosts (users: Array<Users>, categories: Array<ForumCategories>, amount = 200): Array<ForumPosts> {
  const normalPosts = generateNormalPosts(users, categories, amount)
  const categoryBased = generateCategoryBasedAnnouncements(users, categories)
  const globalAnnouncements = generateGlobalAnnouncements(users, categories)

  return [...normalPosts, ...categoryBased, ...globalAnnouncements]
}

function generateNormalPosts (
  users: Array<Users>,
  categories: Array<ForumCategories>,
  amount = 200
): Array<ForumPosts> {
  return Array.from({ length: amount }, () => {
    const createdTimestamp = faker.date.between(dayjs().subtract(8, 'years').toDate(), new Date())
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
      forum_post_is_category_based_announcement: false,
      forum_post_deactivated_timestamp: !isActive ? lastActiveDate.toISOString() : null,
      forum_post_is_global_announcement: false
    }

    return post
  })
}

function generateCategoryBasedAnnouncements (
  users: Array<Users>,
  categories: Array<ForumCategories>,
  amount = 100
): Array<ForumPosts> {
  const deactivatedCategoryBasedAnnouncements = Array.from({ length: amount }, () => {
    const createdTimestamp = faker.date.between(dayjs().subtract(8, 'years').toDate(), new Date())

    const post: ForumPosts = {
      forum_post_id: nanoid(NANOID_LENGTH),
      forum_post_name: faker.commerce.productName(),
      user_id: faker.helpers.arrayElement(users).user_id,
      forum_category_id: faker.helpers.arrayElement(categories).forum_category_id,
      forum_post_content: faker.lorem.paragraph(100).replaceAll(',', ''),
      forum_post_is_active: false,
      forum_post_created_timestamp: createdTimestamp.toISOString(),
      forum_post_last_active_timestamp: createdTimestamp.toISOString(),
      forum_post_is_category_based_announcement: true,
      forum_post_deactivated_timestamp: faker.date.between(createdTimestamp, new Date()).toISOString(),
      forum_post_is_global_announcement: false
    }

    return post
  })

  const activeCategoryBasedAnnouncements = categories.map(c => {
    return Array.from({ length: faker.datatype.number({ min: 1, max: 5 }) }, () => {
      const createdTimestamp = faker.date.between(dayjs().subtract(8, 'years').toDate(), new Date())

      const post: ForumPosts = {
        forum_post_id: nanoid(NANOID_LENGTH),
        forum_post_name: faker.commerce.productName(),
        user_id: faker.helpers.arrayElement(users).user_id,
        forum_category_id: c.forum_category_id,
        forum_post_content: faker.lorem.paragraph(100).replaceAll(',', ''),
        forum_post_is_active: true,
        forum_post_created_timestamp: createdTimestamp.toISOString(),
        forum_post_last_active_timestamp: createdTimestamp.toISOString(),
        forum_post_is_category_based_announcement: true,
        forum_post_deactivated_timestamp: null,
        forum_post_is_global_announcement: false
      }

      return post
    })
  }).flat()

  return [...deactivatedCategoryBasedAnnouncements, ...activeCategoryBasedAnnouncements]
}

function generateGlobalAnnouncements (
  users: Array<Users>,
  categories: Array<ForumCategories>,
  amount = 100
): Array<ForumPosts> {
  const globalAnnouncement = categories.find((v) => v.forum_category_representative_id === 'global-announcements')

  if (globalAnnouncement == null) {
    throw new Error('global announcement cannot be found')
  }

  const activeGlobalAnnouncements = Array.from({ length: faker.datatype.number({ min: 1, max: 5 }) }, () => {
    const createdTimestamp = faker.date.between(dayjs().subtract(8, 'years').toDate(), new Date())

    const post: ForumPosts = {
      forum_post_id: nanoid(NANOID_LENGTH),
      forum_post_name: faker.commerce.productName(),
      user_id: faker.helpers.arrayElement(users).user_id,
      forum_category_id: globalAnnouncement.forum_category_id,
      forum_post_content: faker.lorem.paragraph(100).replaceAll(',', ''),
      forum_post_is_active: true,
      forum_post_created_timestamp: createdTimestamp.toISOString(),
      forum_post_last_active_timestamp: createdTimestamp.toISOString(),
      forum_post_is_category_based_announcement: false,
      forum_post_deactivated_timestamp: null,
      forum_post_is_global_announcement: true
    }

    return post
  })

  const deactivatedGlobalAnnouncements = Array.from({ length: amount }, () => {
    const createdTimestamp = faker.date.between(dayjs().subtract(8, 'years').toDate(), new Date())
    const lastActiveTimestamp = faker.date.between(createdTimestamp, dayjs(createdTimestamp).add(1, 'year').toDate())

    const post: ForumPosts = {
      forum_post_id: nanoid(NANOID_LENGTH),
      forum_post_name: faker.commerce.productName(),
      user_id: faker.helpers.arrayElement(users).user_id,
      forum_category_id: globalAnnouncement.forum_category_id,
      forum_post_content: faker.lorem.paragraph(100).replaceAll(',', ''),
      forum_post_is_active: false,
      forum_post_created_timestamp: createdTimestamp.toISOString(),
      forum_post_last_active_timestamp: lastActiveTimestamp.toISOString(),
      forum_post_is_category_based_announcement: false,
      forum_post_deactivated_timestamp: lastActiveTimestamp.toISOString(),
      forum_post_is_global_announcement: true
    }

    return post
  })

  return [...deactivatedGlobalAnnouncements, ...activeGlobalAnnouncements]
}
