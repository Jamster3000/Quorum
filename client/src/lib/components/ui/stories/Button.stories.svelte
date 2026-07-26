<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import { IconHexagonFilled } from '@tabler/icons-svelte-runes';
    import Button from '../Button.svelte';
    import Card from '../Card.svelte';

    const { Story } = defineMeta({
        title: 'UI/Button',
        component: Button,
    });
</script>

{#snippet template(args)}
    <Card height="100%">
        <Button {...args}>
            Click me
        </Button>
    </Card>
{/snippet}

{#snippet icon_template(args)}
    <Card height="100%">
        <Button {...args}>
            <IconHexagonFilled size={150}/> 
        </Button>
    </Card>
{/snippet}

<Story name="Primary" args={{ variant: 'primary', fontSize: 'medium' }} {template} />
<Story name="Secondary" args={{ variant: 'secondary', fontSize: 'medium' }} {template} />
<Story name="Transparent" args={{ variant: 'transparent', fontSize: 'medium' }} {template} />
<Story name="Disabled" args={{ variant: 'primary', fontSize: 'medium', disabled: true }} {template} />
<Story name="Anchor Link" args={{ variant: 'primary', fontSize: 'medium', href: "https:www.example.com" }} {template} />
<Story name="Icon Only" args={{ variant: 'transparent', iconOnly: true }} template={icon_template} />

<!--Font isn't loaded for storybook-->
