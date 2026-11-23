import adapter from "@sveltejs/adapter-static";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    kit: {
        adapter: adapter({
            fallback: "index.html",
        }),
        alias: {
            $lib: "src/lib",
            $components: "src/components",
            $utils: "src/lib/utils",
            $stores: "src/lib/stores"
        },
    },
};

export default config;