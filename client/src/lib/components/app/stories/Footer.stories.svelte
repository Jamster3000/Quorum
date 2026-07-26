<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import Footer from '../Footer.svelte';

    const { Story } = defineMeta({
        title: 'App/Footer',
        tags: ['autodocs'],
        component: Footer,
    });
</script>

{#snippet template(args)}
    <Footer {...args}/>
{/snippet}

<Story name="Footer" {template}/>