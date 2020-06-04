const path = require('path');
const webpack = require('webpack');

module.exports = {
  entry: './frontend/',
  output: { path: path.join(__dirname, "static/assets"), filename: 'bundle.js' },
  mode: 'development',
  module: {
    rules: [
      {
        test: /.jsx?$/,
        loader: 'babel-loader',
        exclude: /node_modules/,
        query: {
          presets: ['@babel/env', '@babel/react']
        }
      }
    ]
  },
  devServer: {
    // This is where webpack-dev-server serves your bundle
    // which is created in memory. In this example it will be:
    //   http://localhost/assets/bundle.js
    contentBase: './static/',
    // Make webpack-dev-server live-reload when your
    // shell page changes
    watchContentBase: true,
    publicPath: '/assets/'
  },
};
