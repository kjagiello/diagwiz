const sveltePreprocess = require("svelte-preprocess");
const static = require("@sveltejs/adapter-static");
const pkg = require("./package.json");
const ViteRsw = require("vite-plugin-rsw").default;
const lezer = require("lezer-generator/rollup").lezer;

/** @type {import('@sveltejs/kit').Config} */
module.exports = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: sveltePreprocess(),
  kit: {
    // Build a static version
    adapter: static(),

    // hydrate the <div id="svelte"> element in src/app.html
    target: "#svelte",

    vite: {
      ssr: {
        noExternal: Object.keys(pkg.dependencies || {}),
      },
      plugins: [
        // Rust support
        ViteRsw({
          mode: "release",
          crates: ["diagwiz"],
        }),

        // Lezer grammar support
        lezer(),
      ],
    },
  },
};
