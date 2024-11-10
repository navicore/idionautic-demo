const path = require('path');

module.exports = {
  entry: './src/app.js',  // The main entry point for your app
  output: {
    path: path.resolve(__dirname, '../static'), // Output to static dir in Rust project
    filename: 'app.js',  // Bundle name
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
        },
      },
    ],
  },
  mode: 'development',  // Set to 'production' for production builds
  devServer: {
    static: {
      directory: path.join(__dirname, 'src'),
    },
    compress: true,
    port: 8080,
  },
};

