const path = require('path');
const webpack = require('webpack');

module.exports = {
  entry: './frontend/index.js',
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
};
