const { createAppBuilder, appBuilderBuild } = require("./index.node");

process.on('SIGINT', function() {
  process.exit(0);
});

class App {

  constructor(cli) {
    this.internal = createAppBuilder(cli);
  }

  async run() {
    await appBuilderBuild.call(this.internal);
  }
}

module.exports = { App };
