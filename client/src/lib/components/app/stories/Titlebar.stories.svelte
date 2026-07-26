<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import Titlebar from '../Titlebar.svelte';

    const { Story } = defineMeta({
        title: 'App/Titlebar',
        component: Titlebar,
    });
</script>

{#snippet template(args)}
    <Titlebar {...args}/>
{/snippet}

<Story name="Titlebar" {template}/>