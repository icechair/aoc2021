import { parse } from "https://deno.land/std@0.118.0/flags/mod.ts";
import { readLines } from "https://deno.land/std@0.118.0/io/buffer.ts";
import { assert } from "https://deno.land/std@0.118.0/testing/asserts.ts";
const add_snail = (a: Array<any>, b: Array<any>) => {
  return [a, b];
};
const Snail = (line: string) => JSON.parse(line);

const reduce_pair = (snail: Array<any>, depth: number) => {};

let a = [1, 2];
let b = [[3, 4], 5];
let c = add_snail(a, b);
assert(JSON.stringify(c) == "[[1,2],[[3,4],5]]", "adding should work");

const { part, input } = parse(Deno.args);
const file = await Deno.open(input);

for await (let line of readLines(file)) {
  //  console.log(JSON.parse(line));
}
