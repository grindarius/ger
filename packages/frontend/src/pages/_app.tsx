import type { NextPage } from 'next'
import type { AppProps } from 'next/app'
import type { ReactElement, ReactNode } from 'react'

import Menu from '@/components/menu'

import '@/styles/globals.css'

export type NextPageWithLayout<P = {}, IP = P> = NextPage<P, IP> & {
  getLayout?: (page: ReactElement) => ReactNode
}

type AppPropsWithLayout = AppProps & {
  Component: NextPageWithLayout
}

export default function App ({ Component, pageProps }: AppPropsWithLayout): ReactNode {
  // Use the layout defined at the page level, if available
  // if not, use menu page
  const getLayout = Component.getLayout ?? ((page) => <Menu>{page}</Menu>)

  return getLayout(<Component {...pageProps} />)
}
