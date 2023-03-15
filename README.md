Teo Node.js
==========

Run Teo server and write custom callbacks with Node.js.

## Installation

```sh
npm install @teocloud/teo
```

## Example

```javascript
const { App } = require("./index.js")

async function main() {
  const app = new App();
  await app.run();
}

main();
```
