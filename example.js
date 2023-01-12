const { App } = require("./index.js")

async function main() {
  const app = new App();
  await app.run();
}

main();
