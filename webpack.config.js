const path = require('path')

module.exports = {
  entry: ['./src/index.js'],
  output: {
    filename: 'app.js',
    path: path.resolve(__dirname, 'build')
  },
  devServer: {
    contentBase: path.join(__dirname, 'public'),
    compress: true,
    disableHostCheck: true,
    host: '0.0.0.0',
    port: 8080,
    watchOptions: {
      aggregateTimeout: 500,
      poll: 1000
    }
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: 'babel-loader'
      },
      {
        test: /\.css$/,
        use: [
          {
            loader: 'style-loader'
          },
          {
            loader: 'css-loader',
            options: {
              modules: true
            }
          }
        ]
      }
    ]
  }
}
