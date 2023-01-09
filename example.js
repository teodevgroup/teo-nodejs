const { AppBuilder } = require("./index.js")

async function main() {
  const appBuilder = new AppBuilder();
  appBuilder.load();
  const app = await appBuilder.build();
  await app.run();
}

main();
