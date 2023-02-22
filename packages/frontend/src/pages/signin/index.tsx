import Head from 'next/head'
import React from 'react'

export default function Signin (): JSX.Element {
  return (
    <>
      <Head>
        <title>Signin • ger</title>
        <meta name="description" content="reg spelled backwards" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <main className="min-h-screen hero bg-base-200">
        <div className="flex-col lg:flex-row-reverse hero-content">
          <div className="text-center lg:text-left">
            <h1 className="text-5xl font-bold">Signin</h1>
          </div>
          <div className="flex-shrink-0 w-full max-w-sm shadow-2xl card bg-base-100">
            <div className="card-body">
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Username or email</span>
                </label>
                <input type="text" placeholder="email" className="input input-bordered" />
              </div>
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Password</span>
                </label>
                <input type="text" placeholder="password" className="input input-bordered" />
                <label className="label">
                  <a href="#" className="label-text-alt link link-hover">Forgot password?</a>
                </label>
              </div>
              <div className="mt-6 form-control">
                <button className="btn btn-primary">Login</button>
              </div>
            </div>
          </div>
        </div>
      </main>
    </>
  )
}
