#!/usr/bin/env node

const { App } = require("./index.js")

const app = App.withCli(true)
app.run()
