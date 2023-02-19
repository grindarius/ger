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
            <thead>
              <tr>
                <td>Topic</td>
                <td>Viewers</td>
                <td>Replies</td>
                <td>Views</td>
                <td>Activity</td>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>
                  <Link className="font-bold link link-hover" href="/forum/posts/123456">How to dive the web.</Link>
                  <div className="flex flex-row">
                    <Link className="text-sm opacity-75 link link-hover" href="/forum/users/grindarius">
                      grindarius
                    </Link>
                    &nbsp;•&nbsp;
                    <div className="text-sm opacity-50">
                      February 6, 2022 19:12
                    </div>
                  </div>
                </td>
                <td>
                  <div className="flex flex-row">
                    <Link className="w-8 h-8 rounded-full ring-white ring-2 overflow-hidden" href="/forum/users/grindarius" passHref>
                      <img className="w-8 h-8" src="https://upload.wikimedia.org/wikipedia/commons/b/b6/Image_created_with_a_mobile_phone.png" alt="what" />
                    </Link>
                    <Link className="w-8 h-8 rounded-full ring-white ring-2 overflow-hidden" href="/forum/users/grindarius" passHref>
                      <img className="w-8 h-8" src="https://upload.wikimedia.org/wikipedia/commons/b/b6/Image_created_with_a_mobile_phone.png" alt="what" />
                    </Link>
                    <Link className="w-8 h-8 rounded-full ring-white ring-2 overflow-hidden" href="/forum/users/grindarius" passHref>
                      <img className="w-8 h-8" src="https://upload.wikimedia.org/wikipedia/commons/b/b6/Image_created_with_a_mobile_phone.png" alt="what" />
                    </Link>
                  </div>
                </td>
                <td>10</td>
                <td>10</td>
                <td>1h</td>
              </tr>
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
