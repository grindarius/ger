import dayjs from 'dayjs'
import timezone from 'dayjs/plugin/timezone.js'
import utc from 'dayjs/plugin/utc.js'
import { resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import pg from 'pg'
import { pino } from 'pino'

import { faker } from '@faker-js/faker'

import { insert, saveToFile } from './generals.js'
import { generateAcademicYears } from './tables/academic-years/academic-years.js'
import { generateCurriculums } from './tables/curriculums/curriculums.js'
import { generateFaculties } from './tables/curriculums/faculties.js'
import { generateMajors } from './tables/curriculums/majors.js'
import { generateForumCategories } from './tables/forums/forum-categories.js'
import { generateForumPostReplies } from './tables/forums/forum-post-replies.js'
import { generateForumPostViews } from './tables/forums/forum-post-views.js'
import { generateForumPostVotes } from './tables/forums/forum-post-votes.js'
import { generateForumPosts } from './tables/forums/forum-posts.js'
import { generateUsers } from './tables/users/admins.js'
import { generateProfessors } from './tables/users/professors.js'
import { generateStudents } from './tables/users/students.js'
import { generateForumPostReplyVotes } from './tables/forums/forum-post-reply-votes.js'

import './env.js'

/* eslint-disable-next-line @typescript-eslint/naming-convention */
const __dirname = fileURLToPath(new URL('.', import.meta.url))

faker.seed(331503)

dayjs.extend(utc)
dayjs.extend(timezone)

dayjs.tz.setDefault('Asia/Bangkok')

export const logger = pino({
  transport: {
    target: 'pino-pretty',
    options: {
      colorize: true
    }
  }
})

const username = process.env['GER_POSTGRES_USERNAME'] ?? ''
const password = process.env['GER_POSTGRES_PASSWORD'] ?? ''
const host = process.env['GER_POSTGRES_HOST'] ?? ''
const port = process.env['GER_POSTGRES_PORT'] != null || process.env['GER_POSTGRES_PORT'] !== '' ? Number(process.env['GER_POSTGRES_PORT']) : 5432
const databaseName = process.env['GER_POSTGRES_DATABASE_NAME'] ?? ''

logger.info(resolve(__dirname, '..', '..', '..', '.env.local'))
logger.info(username)
logger.info(password)
logger.info(host)
logger.info(port)
logger.info(databaseName)

const pool = new pg.Pool({
  user: username,
  password,
  host,
  port,
  database: databaseName
})

const faculties = generateFaculties()
const curriculums = generateCurriculums(faculties)
const majors = generateMajors(curriculums)
const academicYears = generateAcademicYears(2000, 2022)

const users = generateUsers()
const professors = generateProfessors()
const students = generateStudents(majors, academicYears, professors.map(p => p[1]))

users.push(...professors.map(p => p[0]))
users.push(...students.map(s => s[0]))

const forumCategories = generateForumCategories(users)
const forumPosts = generateForumPosts(users, forumCategories)
const forumViews = generateForumPostViews(users, forumPosts)
const forumVotes = generateForumPostVotes(forumPosts, forumViews)
const forumPostReplies = generateForumPostReplies(users, forumPosts)
const forumPostReplyVotes = generateForumPostReplyVotes(users, forumPostReplies)

await Promise.all([
  saveToFile(faculties, 'faculties.json'),
  saveToFile(curriculums, 'curriculums.json'),
  saveToFile(majors, 'majors.json'),
  saveToFile(academicYears, 'academic-years.json'),
  saveToFile(users, 'users.json'),
  saveToFile(professors.map(p => p[1]), 'professors.json'),
  saveToFile(students.map(s => s[1]), 'students.json'),
  saveToFile(forumCategories, 'forum-categories.json'),
  saveToFile(forumPosts, 'forum-posts.json'),
  saveToFile(forumViews, 'forum-post-views.json'),
  saveToFile(forumVotes, 'forum-post-votes.json'),
  saveToFile(forumPostReplies, 'forum-post-replies.json'),
  saveToFile(forumPostReplyVotes, 'forum-post-reply-votes.json')
])

await insert(faculties, 'faculties', pool)
await insert(curriculums, 'curriculums', pool)
await insert(majors, 'majors', pool)

await insert(academicYears, 'academic_years', pool)

await insert(users, 'users', pool)
await insert(professors.map(p => p[1]), 'professors', pool)
await insert(students.map(s => s[1]), 'students', pool)

await insert(forumCategories, 'forum_categories', pool)
await insert(forumPosts, 'forum_posts', pool)
await insert(forumViews, 'forum_post_views', pool)
await insert(forumVotes, 'forum_post_votes', pool)
await insert(forumPostReplies, 'forum_post_replies', pool)
await insert(forumPostReplyVotes, 'forum_post_reply_votes', pool)
