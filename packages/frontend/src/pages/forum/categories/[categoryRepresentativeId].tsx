import ky from 'ky-universal'
import type { GetServerSidePropsContext, GetServerSidePropsResult } from 'next'
import Head from 'next/head'
import { useRouter } from 'next/router'
import type { ParsedUrlQuery } from 'querystring'
import { useState } from 'react'

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
    by: 'latest_activity'
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
    by: 'latest_activity',
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
  context: GetServerSidePropsContext<{ categoryRepresentativeId: string, page?: string }>
): Promise<GetServerSidePropsResult<CategoryOptions>> {
  const id = context?.params?.categoryRepresentativeId ?? ''
  const page = context?.params?.page == null ? 1 : Number(context?.params?.page)

  const initialPosts = await getPostsByCategory(id, page)
  const metadata = await getCategoryMetadata(id)

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

function getStringParam (query: ParsedUrlQuery, keyName: string): string {
  if (query?.[keyName] == null) {
    return ''
  }

  if (Array.isArray(query[keyName])) {
    const firstString = query?.[keyName]?.[0]
    return firstString ?? ''
  }

  return query?.[keyName] as string ?? ''
}

function Category ({ initialPosts, metadata }: CategoryOptions): JSX.Element {
  const headTitle = `${metadata.name} • ger`
  const router = useRouter()
  const initialPage = router.query?.['page'] == null ? 1 : Number(router.query?.['page'])
  const representativeId = getStringParam(router.query, 'categoryRepresentativeId')

  const [page, setPage] = useState<number>(initialPage)
  const [posts, setPosts] = useState<GetPostListResponseBody>(initialPosts)

  const nextPage = (): void => {
    const next = page + 1
    setPage(next)
    getPostsByCategory(representativeId, next).then(p => {
      setPosts(p)
      // eslint-disable-next-line
      router.push(new URL(`/forum/categories/${representativeId}?page=${next}`, 'http://localhost:3000'), undefined, { shallow: true })
    }).catch(e => {
      console.error(e)
      setPosts({ posts: [] })
    })
  }

  const previousPage = (): void => {
    const prev = page - 1

    if (prev <= 0) {
      return
    }

    setPage(prev)
    getPostsByCategory(representativeId, prev).then(p => {
      setPosts(p)
      // eslint-disable-next-line
      router.push(new URL(`/forum/categories/${representativeId}?page=${prev}`, 'http://localhost:3000'), undefined, { shallow: true })
    }).catch(e => {
      console.error(e)
      setPosts({ posts: [] })
    })
  }

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
        <h3 className="text-2xl text-current">
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
                posts.posts.map(a => {
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

        <div className="btn-group w-full justify-center">
          <button className="btn" onClick={previousPage}>Previous</button>
          <button className="btn" onClick={nextPage}>Next</button>
        </div>
      </main>
    </>
  )
}

export default Category
