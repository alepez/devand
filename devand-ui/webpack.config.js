const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const configurations = {
  production: {
    distPath: path.resolve(__dirname, "../devand-www/static/ui"),
    publicPath: '/public/ui/',
    cargoFeatures: [],
  },
  development: {
    distPath: path.resolve(__dirname, "dist"),
    publicPath: '',
    cargoFeatures: ["mock_http"],
  }
};

const argsFromCargoFeatures = (features) => features.map(x => `--features=${x}`).join(" ")

module.exports = (env, argv) => {
  const {distPath, publicPath, cargoFeatures} = configurations[argv.mode];

  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      host: '0.0.0.0',
      port: 8001,
      historyApiFallback: true,
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "devand.js",
      publicPath,
      webassemblyModuleFilename: "devand.wasm"
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: [
            'style-loader',
            'css-loader',
            'sass-loader',
          ],
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin([
        {from: './static', to: distPath}
      ]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript -- " + argsFromCargoFeatures(cargoFeatures),
      })
    ],
    watch: argv.mode !== 'production'
  };
};
