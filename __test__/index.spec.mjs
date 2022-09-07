import test from 'ava'

import { Clipboard } from '../index.js'

test('setText and getText work', (t) => {
  const clipboard = new Clipboard()
  clipboard.setText('@enpitsulin/arboard')
  t.is(clipboard.getText(), '@enpitsulin/arboard')
})
