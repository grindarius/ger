import React from 'react'

import { LockIcon } from '@primer/octicons-react'

interface RowUsernameOptions {
  isActive: boolean
  name: string
}

export default function RowUsername ({ isActive, name }: RowUsernameOptions): JSX.Element {
  if (!isActive) {
    return (
      <><LockIcon size={16}/>&nbsp;{name}</>
    )
  }

  return <>{name}</>
}
