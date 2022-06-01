module.exports = {
  content: [
    "./public/**/*.html",
    "./public/**/*.js",
    "./public/**/*.css",
    "./src/**/*.rs",
    "./src/**/*.html",
    "./src/**/*.css",
    "./index.html",
  ],
  theme: {},
  variants: {},
  plugins: [require("@tailwindcss/typography")],
};
