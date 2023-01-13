const { App } = require("./index.js")

async function main() {
  const app = new App(true);
  await app.run();
}

main();
