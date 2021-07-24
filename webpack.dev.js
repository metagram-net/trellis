const path = require("path");
const { merge } = require("webpack-merge");
const common = require("./webpack.common.js");

module.exports = merge(common, {
  devtool: "inline-source-map",
  devServer: {
    contentBase: path.resolve(__dirname, "dist"),
    port: 3000,
    proxy: [
      {
        context: ["/api", "/authenticate"],
        target: "http://localhost:8000",
        secure: false,
      },
    ],
  },
  mode: "development",
});
