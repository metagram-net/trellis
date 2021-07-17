const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const crate = path.resolve(__dirname, ".");
const dist = path.resolve(__dirname, "dist");
module.exports = (env, argv) => ({
  devServer: {
    contentBase: dist,
    compress: argv.mode === "production",
    port: 8000,
  },
  entry: "./index.js",
  output: {
    filename: "index.js",
    path: dist,
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: "Trellis",
    }),
    new WasmPackPlugin({
      crateDirectory: crate,
    }),
    new MiniCssExtractPlugin(),
  ],
  mode: argv.mode || "development",
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader", "postcss-loader"],
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
  },
});
