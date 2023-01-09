const { createAppBuilder, appBuilderLoad, appBuilderBuild, appRun } = require("./index.node");

class AppBuilder {

  constructor() {
    this.internal = createAppBuilder();
  }

  load(filename) {
    appBuilderLoad.call(this.internal, filename);
  }

  async build() {
    const appInternal = await appBuilderBuild.call(this.internal);
    return new App(appInternal);
  }
}

class App {

  constructor(internal) {
    this.internal = internal;
  }

  async run() {
    await appRun.call(this.internal);
  }
}

module.exports = { AppBuilder, App };
