import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import Link from 'next/link'
import React from 'react'

import RowUsername from './row-username'

export interface RowOptions {
  id: string
  name: string
  username: string
  createdTimestamp: string
  replyCount: number
  viewCount: number
  lastActiveTimestamp: string
  isActive: boolean
}

export default function Row ({
  id,
  name,
  username,
  createdTimestamp,
  replyCount,
  viewCount,
  lastActiveTimestamp,
  isActive
}: RowOptions): JSX.Element {
  dayjs.extend(relativeTime)

  return (
    <tr>
      <td style={ { width: '800px' } }>
        <Link className="font-bold link link-hover" href={{ pathname: '/forum/posts/[postId]', query: { postId: id } }}>
          <RowUsername isActive={isActive} name={name} />
        </Link>
        <div className="flex flex-row">
          <Link className="text-sm opacity-75 link link-hover" href={{ pathname: '/forum/users/[username]', query: { username } }}>
            {username}
          </Link>
          <p className="text-sm opacity-75">
             &nbsp;•&nbsp;{dayjs(createdTimestamp).format('MMMM D, YYYY HH:mm')}
          </p>
        </div>
      </td>
      <td>{replyCount}</td>
      <td>{viewCount}</td>
      <td>{dayjs(lastActiveTimestamp).fromNow()}</td>
    </tr>
  )
}
