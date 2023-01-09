Teo Node.js
==========

Run Teo server and write custom callbacks with Node.js.

## Installation

```sh
npm install @teocloud/teo
```

## Example

```javascript
const { AppBuilder } = require("./index.js")

async function main() {
  const appBuilder = new AppBuilder();
  appBuilder.load();
  const app = await appBuilder.build();
  await app.run();
}

main();
```
