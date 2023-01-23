import Head from 'next/head'
import Link from 'next/link'

export default function Home (): JSX.Element {
  return (
    <>
      <Head>
        <title>ger</title>
        <meta name="description" content="reg spelled backwards" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className="hero min-h-screen bg-base-200">
        <div className="hero-content text-center">
          <div className="max-w-md">
            <h1 className="text-5xl font-bold">Hello there</h1>
            <p className="py-6">
              Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.
            </p>
            <Link className="btn btn-primary" href="/signin">Signin</Link>
          </div>
        </div>
      </main>
    </>
  )
}
