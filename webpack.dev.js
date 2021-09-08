const path = require("path");
const { merge } = require("webpack-merge");
const common = require("./webpack.common.js");

module.exports = merge(common, {
  devtool: "inline-source-map",
  devServer: {
    contentBase: path.resolve(__dirname, "dist"),
    // Render index.html for unrecognized paths.
    historyApiFallback: true,
    port: 3000,
    proxy: {
      "/api": {
        pathRewrite: { "^/api": "" },
        secure: false,
        target: "http://localhost:8000",
      },
    },
  },
  mode: "development",
});
