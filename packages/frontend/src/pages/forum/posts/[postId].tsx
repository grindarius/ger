import Head from 'next/head'
import { useRouter } from 'next/router'
import React from 'react'

export default function Post (): JSX.Element {
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
