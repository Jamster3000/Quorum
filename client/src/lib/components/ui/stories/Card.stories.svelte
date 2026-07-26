<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import Card from '../Card.svelte';

    const { Story } = defineMeta({
        title: 'UI/Card',
        component: Card,
    });
</script>

{#snippet template(args)}
    <Card {...args}>
        <p>This component is a card</p>
    </Card>
{/snippet}

<Story name="Width" args={{ width: "50%" }} {template} />
<Story name="Auto Width" args={{ width: "auto" }} {template} />
<Story name="Gradient variant" args={{ variant: "gradient" }} {template} />
<Story name="Extra Large Padding" args={{ padding: "xlarge" }} {template} />
<Story name="Left aligned Content" args={{ center: false }} {template} />