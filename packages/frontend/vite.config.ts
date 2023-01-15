import { defineConfig } from 'vite'
import tsconfigPaths from 'vite-tsconfig-paths'

import { qwikVite } from '@builder.io/qwik/optimizer'
import { qwikCity } from '@builder.io/qwik-city/vite'

export default defineConfig(() => {
  return {
    // TODO: qwikCity({ trailingSlash: true }) will be available in qwikcity 0.1
    plugins: [qwikCity(), qwikVite(), tsconfigPaths()],
    preview: {
      headers: {
        'Cache-Control': 'public, max-age=600'
      }
    }
  }
})
