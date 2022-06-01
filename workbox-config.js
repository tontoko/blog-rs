module.exports = {
  globDirectory: "dist/",
  globPatterns: ["**/*.{wasm,js,html,png,json,css}"],
  globIgnores: ["**/node_modules/**/*", "sw.js*", "workbox-*.js*"],
  swDest: "dist/sw.js",
  ignoreURLParametersMatching: [/^utm_/, /^fbclid$/],
};
