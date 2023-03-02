import type { GetServerSidePropsResult } from 'next'
import Head from 'next/head'
import { useRouter } from 'next/router'

export async function getServerSideProps (): Promise<GetServerSidePropsResult<>> 

function Category (): JSX.Element {
  const router = useRouter()
  const { categoryRepresentativeId } = router.query

  return (
    <>
      <Head>
        <title> • ger</title>
        <meta name="description" content="reg spelled backwards" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
    </>
  )
}

export default Category
