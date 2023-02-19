import { exec } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { dirname } from 'node:path'

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

exec('cp ../backend/bindings/* ../frontend/src/types/', { cwd: __dirname }, function (error, stdout, stderr) {
  console.log(stdout)
  console.log(stderr)

  if (error != null) {
    throw error
  }
})
