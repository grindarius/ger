import Head from 'next/head'
import type { ReactElement } from 'react'

import Menu from '@/components/menu'
import type { NextPageWithLayout } from './_app'

const Home: NextPageWithLayout = () => {
  return (
    <>
      <Head>
        <title>ger</title>
        <meta name="description" content="reg spelled backwards" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <h1 className="text-3xl">hi</h1>
      </main>
    </>
  )
}

Home.getLayout = function getLayout (content: ReactElement): JSX.Element {
  return (
    <Menu>{content}</Menu>
  )
}

export default Home
