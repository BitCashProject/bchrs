import * as wasm from "../pkg/wasm";

console.log(wasm.add(1, 5));
console.log(wasm.sub(257, 1));

console.time("js");
let i = 0;
while (true) {
  if (i === 999999999) break;
  i++;
}
console.log(i);
console.timeEnd("js");

console.time("wasm");
console.log(wasm.hello());
console.timeEnd("wasm");
