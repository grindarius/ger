import Link from 'next/link'
import type { ReactElement } from 'react'
import React from 'react'

export default function Sidebar ({ children }: { children: ReactElement }): JSX.Element {
  return (
    <>
      <div className="navbar bg-base-100">
        <div className="flex-none">
          <label htmlFor="main-drawer" className="btn btn-square btn-ghost drawer-button">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              className="inline-block w-5 h-5 stroke-current">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 6h16M4 12h16M4 18h16"></path>
            </svg>
          </label>
        </div>
        <div className="flex-1">
          <Link className="btn btn-ghost normal-case text-xl" href="/">ger</Link>
        </div>
        <div className="flex-none">
          <button className="btn btn-square btn-ghost">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              className="inline-block w-5 h-5 stroke-current">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z">
              </path>
            </svg>
          </button>
        </div>
      </div>
      <div className="drawer">
        <input id="main-drawer" type="checkbox" className="drawer-toggle" />
        <div className="drawer-content">
          { children }
        </div>
        <div className="drawer-side">
          <label htmlFor="main-drawer" className="drawer-overlay"></label>
          <ul className="menu p-4 w-80 bg-base-100 text-base-content">
            <li>
              <Link href="/signin">Signin</Link>
            </li>
          </ul>
        </div>
      </div>
    </>
  )
}