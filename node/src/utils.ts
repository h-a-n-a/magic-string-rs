import { resolve } from 'path'
import { loadBinding } from '@node-rs/helper'

export const getMagicString = (() => {
  let binding = null
  return (): any => {
    if (binding) return binding

    const { MagicString } = loadBinding(
      resolve(__dirname, '../'),
      'magic-string-rs',
      '@napi-rs/magic-string'
    )

    return (binding = MagicString)
  }
})()
