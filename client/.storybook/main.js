const config = {
    "stories": [
        "../src/**/*.stories.@(js|ts|svelte)",
        "../src/lib/components/ui/stories/*.stories.@(js|ts|svelte)"
    ],
    "addons": [
        "@storybook/addon-svelte-csf",
        "@chromatic-com/storybook",
        "@storybook/addon-vitest",
        "@storybook/addon-docs",
        {
            name: "@storybook/addon-a11y",
            options: {
                rules: {
                    'color-contrast': {
                        enabled: false,
                    },
                },
            },
        },
    ],
    "framework": "@storybook/sveltekit",
    staticDirs: ['../static'],
};
export default config;