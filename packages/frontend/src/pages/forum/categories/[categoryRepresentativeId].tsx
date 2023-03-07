import ky from 'ky-universal'
import type { GetServerSidePropsContext, GetServerSidePropsResult } from 'next'
import Head from 'next/head'

import Row from '@/components/row'
import type { GetCategoryResponseBody } from '@/types/GetCategoryResponseBody'
import type { GetPostListRequestQueries } from '@/types/GetPostListRequestQueries'
import type { GetPostListResponseBody } from '@/types/GetPostListResponseBody'

async function getCategoryMetadata (categoryRepresentativeId: string): Promise<GetCategoryResponseBody> {
  try {
    const response = await ky.get(`http://127.0.0.1:5155/forum/categories/${categoryRepresentativeId}`).json<GetCategoryResponseBody>()
    return response
  } catch (e) {
    console.error(e)
    return {
      id: '',
      name: '',
      representative_id: '',
      description: '',
      color_theme: ''
    }
  }
}

async function getCategoryAnnouncements (categoryRepresentativeId: string): Promise<GetPostListResponseBody> {
  const queries: GetPostListRequestQueries = {
    page: 1,
    page_size: 10,
    category_representative_id: categoryRepresentativeId,
    announcement: false,
    category_based_announcement: true,
    order: 'desc',
    by: 'time'
  }

  const searchParams = new URLSearchParams(queries as Record<string, string>)

  try {
    const response = await ky.get('http://127.0.0.1:5155/forum/posts', {
      searchParams
    }).json<GetPostListResponseBody>()

    return response
  } catch (e) {
    console.error(e)
    return {
      posts: []
    }
  }
}

async function getPostsByCategory (categoryRepresentativeId: string, page: number): Promise<GetPostListResponseBody> {
  const queries: GetPostListRequestQueries = {
    page,
    page_size: 30,
    category_representative_id: categoryRepresentativeId,
    announcement: false,
    order: 'desc',
    by: 'time',
    category_based_announcement: false
  }

  const searchParams = new URLSearchParams(queries as Record<string, string>)

  try {
    const response = await ky.get('http://127.0.0.1:5155/forum/posts', {
      searchParams
    }).json<GetPostListResponseBody>()

    return response
  } catch (e) {
    console.error(e)
    return {
      posts: []
    }
  }
}

export async function getServerSideProps (
  context: GetServerSidePropsContext<{ categoryRepresentativeId: string }>
): Promise<GetServerSidePropsResult<CategoryOptions>> {
  const initialPosts = await getPostsByCategory(context.params?.categoryRepresentativeId ?? '', 1)
  const metadata = await getCategoryMetadata(context.params?.categoryRepresentativeId ?? '')

  return {
    props: {
      initialPosts,
      metadata
    }
  }
}

interface CategoryOptions {
  initialPosts: GetPostListResponseBody
  metadata: GetCategoryResponseBody
}

function Category ({ initialPosts, metadata }: CategoryOptions): JSX.Element {
  const headTitle = `${metadata.name} • ger`

  return (
    <>
      <Head>
        <title>{headTitle}</title>
        <meta name="description" content="reg spelled backwards" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <main className="container mx-auto">
        <h3 className="text-4xl text-current font-bold mb-4">
          {metadata.name}
        </h3>
        <h3 className="text-xl text-current">
          {metadata.description}
        </h3>
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
                initialPosts.posts.map(a => {
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
      </main>
    </>
  )
}

export default Category
