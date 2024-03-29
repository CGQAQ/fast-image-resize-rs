import b from 'benny'
import * as fs from "fs";
import * as path from "path";

import sharp from 'sharp'

import { resize } from '../index'

// const xx = fs.readFileSync(path.join(__dirname, "../__test__/test.jpg"));
const xx = fs.readFileSync(path.join(__dirname, "./pexels-martin-damboldt-814499.jpg"));

function benchResize() {
    resize(xx, 3000, 2000);
}

async function benchSharp() {
    return sharp(xx).resize(3000, 2000).toBuffer();
}


async function run() {
    await b.suite(
        'resize image size',
        b.add("rust resize", () => {
            benchResize();
        }),
        b.cycle(),
        b.add("Sharp", async () => {
            await benchSharp();
        }),
        b.cycle(),
        b.complete(),
    )
}

run().catch((e) => {
    console.error(e)
})
