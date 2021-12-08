import { resolve } from 'path'
import { loadBinding } from '@node-rs/helper'

export const getMagicString = (): any => {
  const { MagicString } = loadBinding(
    resolve(__dirname, '../'),
    'magic-string-rs',
    'magic-string-rs'
  )
  return MagicString
}
