<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import Header from '../Header.svelte';

    const { Story } = defineMeta({
        title: 'App/Header',
        component: Header,
    });
</script>

{#snippet template(args)}
    <Header {...args}/>
{/snippet}

<Story name="Header" {template}/>