Teo Node.js
==========

Run Teo server and write custom callbacks and entities with Node.js.

## Installation

```sh
npm install @teocloud/teo
```

## Example

Write with JavaScript:

```javascript
const { App } = require("@teocloud/teo")

const app = new App()
app.validate("oddLength", (s) => s.length % 2 == 0)
app.run()

```

Write with TypeScript:

```typescript
import { App } from "@teocloud/teo"

const app = new App()
app.validate("oddLength", (s: string) => s.length % 2 == 0)
app.run()

```
