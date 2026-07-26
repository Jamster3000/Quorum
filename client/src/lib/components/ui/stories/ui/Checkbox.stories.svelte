<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import Checkbox from '../../Checkbox.svelte';
    import Card from '../../Card.svelte'

    const { Story } = defineMeta({
        title: 'UI/Checkbox',
        component: Checkbox,
    });
</script>

{#snippet template(args)}
    <Card height="100%">
        <Checkbox {...args}/>
    </Card>
{/snippet}

<Story name="Label" args={{ label: "This checkbox is for your safty"}} {template}/>
<Story name="Disabled" args={{ disabled: true}} {template}/>
<Story name="error" args={{ error: "This is an error"}} {template}/>