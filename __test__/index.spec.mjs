import test from 'ava'

import { Clipboard } from '../index.js'

test('setText and getText work', (t) => {
  const clipboard = new Clipboard()
  clipboard.setText('@enpitsulin/arboard')
  t.is(clipboard.getText(), '@enpitsulin/arboard')
})

test('getImage and setImage work', (t) => {
  const clipboard = new Clipboard()
  const imageBase64 = 'Hh4eAB4eHgAeHh4AHh4eAA=='
  const imageBuffer = Buffer.from(imageBase64, 'base64')

  clipboard.setImage(2, 2, imageBuffer)
  const buffer = clipboard.getImage()
  t.truthy(buffer)
  t.is(buffer.bytes.toString('base64'), imageBase64)
})
