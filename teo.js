#!/usr/bin/env node

const { App } = require("./index.js")
const app = new App();
app.beforeServerStart(async () => {})
app.run()
