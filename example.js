const { App } = require("./index.js")

async function main() {
  const app = new App();
  app.load();
  await app.run();
}

main();