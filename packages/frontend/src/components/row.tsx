import Link from 'next/link'
import React from 'react'

export interface RowOptions {
  name: string
  username: string
  postDate: string
  users: Array<{ userId: string, username: string }>
  replies: number
  views: number
  activity: string
}

export default function Row ({ name, username, postDate, users, views, replies, activity }: RowOptions): JSX.Element {
  return (
    <tr>
      <td>
        <Link className="font-bold link link-hover" href="/forum/announcements/123456">{name}</Link>
        <div className="flex flex-row">
          <Link className="text-sm opacity-75 link link-hover" href="/forum/users/grindarius">
            {username}
          </Link>
          &nbsp;•&nbsp;
          <div className="text-sm opacity-75">
            {postDate}
          </div>
        </div>
      </td>
      <td>
        <div className="flex flex-row">
          {
            users.map(user => {
              return (
                <Link
                  className="overflow-hidden w-8 h-8 rounded-full ring-2 ring-white"
                  key={user.userId}
                  href={{ pathname: '/forum/users/[username]', query: { username: user.username } }}
                  passHref
                >
                  <img className="w-8 h-8" src="https://upload.wikimedia.org/wikipedia/commons/b/b6/Image_created_with_a_mobile_phone.png" alt="what" />
                </Link>
              )
            })
          }
        </div>
      </td>
      <td>{replies}</td>
      <td>{views}</td>
      <td>{activity}</td>
    </tr>
  )
}
