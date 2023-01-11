import { App } from '.'

async function main() {
  const app = new App();
  app.load();
  await app.run();
}

main();
