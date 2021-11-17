import b from 'benny'
import * as fs from "fs";
import * as path from "path";

import sharp from 'sharp'

import {resize} from '../index'

// const xx = fs.readFileSync(path.join(__dirname, "../__test__/test.jpg"));
const xx = fs.readFileSync(path.join(__dirname, "./pexels-martin-damboldt-814499.jpg"));

function benchResize() {
    resize(xx, 1920, 1080);
}

async function benchSharp() {
    return sharp(xx).resize(1920, 1080).toBuffer();
}


async function run() {
    await b.suite(
        'Add 100',
        b.add("benchResize", () => {
            benchResize();
        }),
        b.cycle(),
        b.add("benchSharp", async () => {
            await benchSharp();
        }),
        b.cycle(),
        b.complete(),
    )
}

run().catch((e) => {
    console.error(e)
})
