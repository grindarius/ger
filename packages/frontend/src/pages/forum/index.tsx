import dayjs from 'dayjs'
import got from 'got'
import type { GetServerSidePropsResult } from 'next'
import Head from 'next/head'
import Link from 'next/link'

import type { GetPostListRequestQueries } from '@/types/GetPostListRequestQueries'
import type { GetPostListResponseBody } from '@/types/GetPostListResponseBody'

export async function getServerSideProps (): Promise<GetServerSidePropsResult<{ announcements: GetPostListResponseBody }>> {
  const queries: GetPostListRequestQueries = {
    announcement: true,
    category_based_announcement: false,
    page: 1
  }

  try {
    const searchParams = new URLSearchParams(queries as Record<string, string>)

    const response = await got.get('http://127.0.0.1:5155/forum/posts', {
      searchParams
    }).json<GetPostListResponseBody>()

    return {
      props: {
        announcements: response
      }
    }
  } catch (e) {
    const response: GetPostListResponseBody = {
      posts: []
    }

    return {
      props: {
        announcements: response
      }
    }
  }
}

function Forum ({ announcements }: { announcements: GetPostListResponseBody }): JSX.Element {
  return (
    <>
      <Head>
        <title>Forum • ger</title>
        <meta name="description" content="reg spelled backwards" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <main className="container mx-auto">
        <h1 className="text-4xl text-current font-bold">Forum</h1>

        <h3 className="text-2xl text-current">Global announcements</h3>
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
                    <tr key={a.id}>
                      <td>
                        <Link className="font-bold link link-hover" href={{ pathname: '/forum/posts/[postId]', query: { postId: a.id } }}>{a.name}</Link>
                        <div className="flex flex-row">
                          <Link className="text-sm opacity-75 link link-hover" href={{ pathname: '/forum/users/[username]', query: { username: a.username } }}>
                            {a.username}
                          </Link>
                          <p className="text-sm opacity-75">
                             &nbsp;•&nbsp;{dayjs(a.created_timestamp).format('MMMM D, YYYY HH:mm')}
                          </p>
                        </div>
                      </td>
                      <td>10</td>
                      <td>{a.view_count}</td>
                      <td>1h</td>
                    </tr>
                  )
                })
              }
            </tbody>
          </table>
        </div>

        <h3 className="text-2xl text-current font-bold">Trending</h3>
        <div className="overflow-x-auto">
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
