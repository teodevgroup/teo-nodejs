const { createAppBuilder, appBuilderLoad, appBuilderBuild } = require("./index.node");

class App {

  constructor() {
    this.internal = createAppBuilder();
  }

  load(filename) {
    appBuilderLoad.call(this.internal, filename);
  }

  async run() {
    await appBuilderBuild.call(this.internal);
  }
}

module.exports = { App };
