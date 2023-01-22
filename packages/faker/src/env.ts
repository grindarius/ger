import { config } from 'dotenv'
import { resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

/* eslint-disable-next-line @typescript-eslint/naming-convention */
const __dirname = fileURLToPath(new URL('.', import.meta.url))

config({
  path: resolve(__dirname, '..', '..', '..', '.env.local')
})
