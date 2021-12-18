const path = require('path')
const webpack = require('webpack')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const RsWatchPlugin = require('./scripts/rs-watch-plugin')

const isProd = process.env.NODE_ENV === 'production'

module.exports = {
  entry: path.join(__dirname, 'src', 'index.ts'),
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  devtool: isProd ? null : 'cheap-source-map',
  mode: isProd ? 'production' : 'development',
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /wasm_bg\.js$/,
        loader: 'string-replace-loader',
        options: {
          search: /module.require/,
          replace: "(module)('require')",
        },
      },
    ],
  },
  // externals: {
  //   '../../wasm/pkg/wasm.js': 'wasm',
  // },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'public', 'index.html'),
    }),
    new RsWatchPlugin({
      sourceRoot: path.resolve(__dirname, '..', 'wasm', 'src'),
      crateRoot: path.resolve(__dirname, '..', 'wasm'),
      mode: isProd ? 'release' : 'debug',
      move: path.resolve(__dirname, 'public', 'wasm'),
    }),
    // new webpack.IgnorePlugin({
    //   resourceRegExp: /^\getStringFromWasm0/,
    //   contextRegExp: /getStringFromWasm0/,
    // }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  stats: 'errors-warnings',
  devServer: {
    client: {
      logging: 'warn',
    },
    port: 3000,
    historyApiFallback: true,
  },
}
