module.exports = {
  root: true,
  env: {
    browser: true,
    es2021: true,
    node: true
  },
  extends: [
    'next/core-web-vitals',
    'standard-with-typescript'
  ],
  ignorePatterns: [
    '.eslintrc.cjs'
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    tsconfigRootDir: __dirname,
    project: [
      './packages/frontend/tsconfig.json',
      './packages/faker/tsconfig.json',
    ],
    ecmaVersion: 2021,
    sourceType: 'module',
    ecmaFeatures: {
      jsx: true
    }
  },
  plugins: [
    'simple-import-sort',
    'unused-imports'
  ],
  rules: {
    'jsx-quotes': ['error', 'prefer-double'],
    '@typescript-eslint/no-unused-vars': ['error'],
    '@typescript-eslint/consistent-type-imports': ['error'],
    '@typescript-eslint/array-type': ['error', {
      default: 'generic',
      readonly: 'generic'
    }],
    'simple-import-sort/imports': [
      'error',
      {
        groups: [['^\\w'], ['^@\\w'], ['^', '^\\.'], ['^\\u0000']]
      }
    ],
    'simple-import-sort/exports': 'error',
    'unused-imports/no-unused-imports': 'error',
    'import/newline-after-import': ['error', { count: 1 }]
  }
}
