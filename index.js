const { createAppBuilder, appBuilderBuild } = require("./index.node");

class App {

  constructor() {
    this.internal = createAppBuilder();
  }

  async run() {
    await appBuilderBuild.call(this.internal);
  }
}

module.exports = { App };
