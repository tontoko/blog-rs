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
  theme: {
    extend: {
      typography: {
        DEFAULT: {
          css: {
            fontFamily: [
              "Helvetica Neue",
              "Arial",
              "Hiragino Kaku Gothic ProN",
              "Hiragino Sans",
              "Meiryo",
              "sans-serif",
            ],
            lineHeight: 2,
            fontFeatureSettings: "palt",
            letterSpacing: "0.1em",
          },
        },
      },
    },
  },
  variants: {},
  plugins: [
    require("@tailwindcss/typography"),
    require("@tailwindcss/line-clamp"),
  ],
};
