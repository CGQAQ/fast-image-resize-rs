import test from 'ava'

import { resize } from '../index'

import * as fs from 'fs'

test("resize", (t) => {

  const xx = fs.readFileSync(__dirname + "/test.jpg")

  const result = resize(xx, 1024, 768);

  t.assert(result.length > 0);
})
