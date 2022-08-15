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
#1 dprint: 12,173 opts/sec, ±17% (mean: 0.082ms, stddev: 0.051ms, 50 samples)
#2 prettier: 450 opts/sec, ±53% (mean: 2.222ms, stddev: 4.229ms, 50 samples)
```
