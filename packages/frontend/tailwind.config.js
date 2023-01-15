/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/pages/**/*.ts*',
    './src/components/**/*.ts*'
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui')
  ],
  daisyui: {
    themes: [
      'forest'
    ]
  }
}
