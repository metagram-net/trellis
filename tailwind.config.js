module.exports = {
  purge: ["./index.js", "./trellis_web/src/**/*.rs"],
  darkMode: "media",
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
