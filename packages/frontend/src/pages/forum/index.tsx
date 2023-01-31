import Head from 'next/head'
import Link from 'next/link'

export default function Forum (): JSX.Element {
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
            <tbody>
              <tr>
                <td>
                  <Link className="font-bold" href="/forum/announcements/123456">How to dive the web.</Link>
                  <div className="text-sm opacity-50">grindarius</div>
                </td>
                <td>Quality Control Specialist</td>
                <td>Blue</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h3 className="text-2xl text-current">Trending</h3>
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
