const { App } = require("./index.js")

async function main() {
  const app = new App();
  app.transform("t1", function (arg0, arg1) {
    console.log("See args: ", arg0, arg1);
    arg1 = arg1 + "suffix"
    return arg1
  })
  await app.run();
}

main();
