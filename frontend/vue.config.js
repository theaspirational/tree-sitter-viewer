// vue.config.js
const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin');


module.exports = {
  publicPath: '/orsl-tool/',
  configureWebpack: {
    plugins: [
      new MonacoWebpackPlugin()
    ]
  }
}