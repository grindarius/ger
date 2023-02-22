import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import ky from 'ky-universal'
import type { GetServerSidePropsResult } from 'next'
import Head from 'next/head'
import { useState } from 'react'

import Row from '@/components/row'
import type { GetCategoriesListRequestQueries } from '@/types/GetCategoriesListRequestQueries'
import type { GetCategoriesListResponseBody } from '@/types/GetCategoriesListResponseBody'
import type { GetPostListRequestQueries } from '@/types/GetPostListRequestQueries'
import type { GetPostListResponseBody } from '@/types/GetPostListResponseBody'

const fetchCategories = async (page: number): Promise<GetCategoriesListResponseBody> => {
  const queries: GetCategoriesListRequestQueries = {
    page
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
    category_based_announcement: false,
    page
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
  const [page, setPage] = useState(1)
  const [announcements, setAnnouncements] = useState(initialAnnouncements)

  dayjs.extend(relativeTime)

  function goToPreviousPage (): void {
    const nextPage = page - 1

    if (nextPage <= 0) {
      return
    }

    setPage(nextPage)
    fetchAnnouncements(nextPage)
      .then(response => {
        setAnnouncements(response)
      })
      .catch(e => {
        console.error(e)
        setAnnouncements({ posts: [] })
      })
  }

  function goToNextPage (): void {
    const nextPage = page + 1
    setPage(nextPage)
    fetchAnnouncements(nextPage)
      .then(response => {
        setAnnouncements(response)
      })
      .catch(e => {
        console.error(e)
        setAnnouncements({ posts: [] })
      })
  }

  return (
    <>
      <Head>
        <title>Forum • ger</title>
        <meta name="description" content="reg spelled backwards" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <main className="container mx-auto">

        <h1 className="text-4xl text-current font-bold">Forum</h1>
        <div className="flex flex-row justify-between">
          <h3 className="text-2xl text-current">Global announcements</h3>
          <div className="flex flex-row btn-group">
            <button className="btn" onClick={goToPreviousPage}>Previous page</button>
            <button className="btn" onClick={goToNextPage}>Next page</button>
          </div>
        </div>
        <div className="overflow-x-auto">
          <table className="table w-full">
            <thead>
              <tr>
                <td>Topic</td>
                <td>Replies</td>
                <td>Views</td>
                <td>Activity</td>
              </tr>
            </thead>
            <tbody>
              {
                announcements.posts.map(a => {
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
                    />
                  )
                })
              }
            </tbody>
          </table>
        </div>

        <h3 className="text-2xl text-current font-bold">Categories</h3>
        <div className="overflow-x-auto">
          {
            initialCategories.categories.map(ic => {
              return (
                <div key={ic.id}>
                  {ic.id}
                </div>
              )
            })
          }
          <table className="table w-full">
            <tbody>
              <tr>
                <td>Cy Ganderton</td>
                <td>Quality Control Specialist</td>
                <td>Blue</td>
              </tr>
            </tbody>
          </table>
        </div>
      </main>
    </>
  )
}

export default Forum
