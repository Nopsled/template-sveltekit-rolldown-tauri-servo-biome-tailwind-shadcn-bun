import adapter from "@sveltejs/adapter-static";

/** @type {import('@sveltejs/kit').Config} */
export default {
  compilerOptions: {
    runes: true,
  },
  adapter: adapter({
    pages: "build",
    assets: "build",
    fallback: "index.html",
  }),
  alias: {
    $lib: "src/lib",
  },
};