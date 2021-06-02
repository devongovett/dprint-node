# dprint-node

A node API for the [dprint](https://dprint.dev) TypeScript and JavaScript code formatter. It's written in Rust for blazing fast speed.

## Usage

Pass a file path and the code to format to `dprint.format`.

```js
const dprint = require('dprint-node');

dprint.format(filePath, code);
```

You can also optionally pass some configuration options as an object to the third parameter. All of the [options listed here](https://dprint.dev/plugins/typescript/config/) are supported.

```js
dprint.format(filePath, code, {
  lineWidth: 100
});
```

## Benchmark

```
$ node bench.js
prettier 216.33 opts/sec (mean: 4.623ms, stddev: 6.7ms, 50 samples)
dprint 6,694.02 opts/sec (mean: 0.149ms, stddev: 0.094ms, 50 samples)
```
