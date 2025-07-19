import adapter from "@sveltejs/adapter-static";

const config = {
  kit: {
    adapter: adapter({
      pages: "build",
      assets: "build",
      fallback: "index.html",
    }),
    alias: {
      "@/*": "./path/to/lib/*",
    },
  },
};

export default config;
