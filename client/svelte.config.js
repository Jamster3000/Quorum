import adapter from "@sveltejs/adapter-static";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    kit: {
        adapter: adapter({
            fallback: "index.html",
        }),
        alias: {
            $lib: "src/lib",
            $app: ".svelte-kit/runtime/app",
        },
    },
};

export default config;