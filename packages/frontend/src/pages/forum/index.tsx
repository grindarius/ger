import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import ky from 'ky-universal'
import type { GetServerSidePropsResult } from 'next'
import Head from 'next/head'
import Link from 'next/link'

import Row from '@/components/row'
import type { GetCategoriesListRequestQueries } from '@/types/GetCategoriesListRequestQueries'
import type { GetCategoriesListResponseBody } from '@/types/GetCategoriesListResponseBody'
import type { GetPostListRequestQueries } from '@/types/GetPostListRequestQueries'
import type { GetPostListResponseBody } from '@/types/GetPostListResponseBody'

const fetchCategories = async (page: number): Promise<GetCategoriesListResponseBody> => {
  const queries: GetCategoriesListRequestQueries = {
    page,
    // way more than the categories amount
    page_size: 30
  }

  const searchParams = new URLSearchParams(queries as Record<string, string>)
  const response = await ky.get('http://127.0.0.1:5155/forum/categories', {
    searchParams
  }).json<GetCategoriesListResponseBody>()

  return response
}

const fetchAnnouncements = async (page: number): Promise<GetPostListResponseBody> => {
  const queries: GetPostListRequestQueries = {
    announcement: true,
    active: true,
    category_based_announcement: false,
    by: 'time',
    order: 'desc',
    page,
    page_size: 5
  }

  const searchParams = new URLSearchParams(queries as Record<string, string>)

  const response = await ky.get('http://127.0.0.1:5155/forum/posts', {
    searchParams
  }).json<GetPostListResponseBody>()

  return response
}

export async function getServerSideProps (): Promise<GetServerSidePropsResult<ForumOptions>> {
  try {
    const initialAnnouncements = await fetchAnnouncements(1)
    const initialCategories = await fetchCategories(1)

    return {
      props: {
        initialAnnouncements,
        initialCategories
      }
    }
  } catch (e) {
    return {
      props: {
        initialAnnouncements: {
          posts: []
        },
        initialCategories: {
          categories: []
        }
      }
    }
  }
}

interface ForumOptions {
  initialAnnouncements: GetPostListResponseBody
  initialCategories: GetCategoriesListResponseBody
}

function Forum ({ initialAnnouncements, initialCategories }: ForumOptions): JSX.Element {
  dayjs.extend(relativeTime)

  return (
    <>
      <Head>
        <title>Forum ??? ger</title>
        <meta name="description" content="reg spelled backwards" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <main className="container mx-auto">
        <h1 className="text-4xl text-current font-bold mb-4">Forum</h1>

        <h3 className="text-2xl text-current">Global announcements</h3>
        <div className="overflow-x-auto mb-4">
          <table className="table w-full">
            <thead>
              <tr>
                <th>Topic</th>
                <th>Replies</th>
                <th>Views</th>
                <th>Activity</th>
              </tr>
            </thead>
            <tbody>
              {
                initialAnnouncements.posts.map(a => {
                  return (
                    <Row
                      key={a.id}
                      id={a.id}
                      name={a.name}
                      username={a.username}
                      createdTimestamp={a.created_timestamp}
                      replyCount={a.reply_count}
                      viewCount={a.view_count}
                      lastActiveTimestamp={a.last_active_timestamp}
                      isActive={a.is_active}
                    />
                  )
                })
              }
            </tbody>
          </table>
        </div>

        <h3 className="text-2xl text-current">Categories</h3>
        <div className="overflow-x-auto">
          <table className="table w-full">
            <thead>
              <tr>
                <th>Category</th>
                <th>Posts count</th>
                <th>Latest Post</th>
              </tr>
            </thead>
            <tbody>
              {
                initialCategories.categories.map(c => {
                  return (
                    <tr key={c.id}>
                      <td style={ { width: '800px' } }>
                        <Link
                          className="font-bold link link-hover"
                          href={
                            {
                              pathname: '/forum/categories/[categoryRepresentativeId]',
                              query: {
                                categoryRepresentativeId: c.representative_id
                              }
                            }
                          }>
                          {c.name}
                        </Link>
                        <br />
                        { c.description }
                      </td>
                      <td>
                        {c.post_count}
                      </td>
                      <td>
                        <Link className="font-bold link link-hover" href={ { pathname: '/forum/posts/[postId]', query: { postId: c.latest_post_id } }}>
                          {c.latest_post_name}
                        </Link>
                        <div className="flex flex-row">
                          <Link className="text-sm opacity-75 link link-hover" href={{ pathname: '/forum/users/[username]', query: { username: c.latest_post_username } }}>
                            {c.latest_post_username}
                          </Link>
                          <p className="text-sm opacity-75">
                            &nbsp;???&nbsp;{dayjs(c.latest_post_created_timestamp).format('MMMM D, YYYY HH:mm')}
                          </p>
                        </div>
                      </td>
                    </tr>
                  )
                })
              }
            </tbody>
          </table>
        </div>
      </main>
    </>
  )
}

export default Forum
