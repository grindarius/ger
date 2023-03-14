import type { GetServerSidePropsContext, GetServerSidePropsResult } from 'next'
import Head from 'next/head'
import { useRouter } from 'next/router'
import React from 'react'

import type { GetPostRepliesResponseBody } from '@/types/GetPostRepliesResponseBody'
import type { GetPostResponseBody } from '@/types/GetPostResponseBody'
import ky from 'ky-universal'

async function getPostMetadata (postId: string): Promise<GetPostResponseBody> {
  try {
    const response = await ky.get(`http://127.0.0.1:5155/forum/posts/${postId}`).json<GetPostResponseBody>()
    return response
  } catch (e) {
    console.error(e)
    return {
      id: '',
      user_id: '',
      username: '',
      name: '',
      content: '',
      created_timestamp: '',
      category_id: '',
      category_name: '',
      category_representative_id: '',
      category_color: '',
      view_count: 0,
      vote_count: 0,
    }
  }
}

async function getPostReplies (postId: string): Promise<GetPostRepliesResponseBody> {

}

export async function getServerSideProps (context: GetServerSidePropsContext<{ postId: string }>): GetServerSidePropsResult<PostOptions> {
  const postId = context?.params?.postId ?? ''

  const results = await Promise.allSettled([
    getPostMetadata(postId),
    getPostReplies(postId)
  ])
}

interface PostOptions {
  metadata: GetPostResponseBody
  replies: GetPostRepliesResponseBody
}

export default function Post ({ metadata, replies }: PostOptions): JSX.Element {
  const router = useRouter()
  const { postId } = router.query

  return (
    <>
      <Head>
        <title>How to surf the web • ger</title>
        <meta name="description" content="reg spelled backwards" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <main className="container mx-auto">
        <h1 className="text-4xl font-bold text-current">How to surf the web.</h1>
        hi this is our first blog post {postId}
      </main>
    </>
  )
}
