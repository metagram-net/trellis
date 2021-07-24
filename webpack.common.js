const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const WebpackPwaManifest = require("webpack-pwa-manifest");
const WorkboxPlugin = require("workbox-webpack-plugin");

module.exports = {
  entry: "./index.js",
  output: {
    filename: "index.js",
    path: path.resolve(__dirname, "dist"),
    publicPath: "/",
  },
  plugins: [
    new WorkboxPlugin.InjectManifest({
      swSrc: path.resolve(__dirname, "service_worker.js"),
    }),
    new WebpackPwaManifest({
      name: "Trellis",
      background_color: "#084908",
      theme_color: "#084908",
      display: "standalone",
      start_url: "/",
      scope: "/",
      icons: [
        {
          src: path.resolve(__dirname, "icon.png"),
          sizes: [192, 512],
        },
      ],
    }),
    new HtmlWebpackPlugin({
      title: "Trellis",
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "trellis-web"),
    }),
    new MiniCssExtractPlugin(),
  ],
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: [
          "style-loader",
          MiniCssExtractPlugin.loader,
          "css-loader",
          "postcss-loader",
        ],
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
  },
};
