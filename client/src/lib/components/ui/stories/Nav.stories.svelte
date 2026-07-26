<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import Nav from '../Nav.svelte';
    import Card from '../Card.svelte';

    const { Story } = defineMeta({
        title: 'Nav',
        component: Nav,
    });
</script>

{#snippet template(args)}
    <Card height="100%">
        <Nav {...args}/>
    </Card>
{/snippet}

<Story name="Links" args={{ links: [{ label: 'Home', href: '/' },{ label: 'Over the rainbow', href: '/rainbow' }] }} {template}/>
