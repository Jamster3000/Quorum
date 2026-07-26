<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import Input from '../../Input.svelte';
    import Card from '../../Card.svelte'

    const { Story } = defineMeta({
        title: 'UI/Input',
        component: Input,
    });
</script>

{#snippet template(args)}
    <Card height="100%">
        <Input {...args}/>
    </Card>
{/snippet}

<Story name="Label" args={{ label: "Please input the meaning of life"}} {template}/>
<Story name="Placeholder" args={{ label: "Please input the meaning of life", placeholder: "The answer isn't what you expect"}} {template}/>
<Story name="Error" args={{ label: "Please input the meaning of life", placeholder: "The answer isn't what you expect", error: "This is an error" }} {template}/>
<Story name="Helper" args={{ label: "Please input the meaning of life", placeholder: "The answer isn't what you expect", helper: "Type something in the box - This is your helper." }} {template}/>
<Story name="Disabled" args={{ label: "Please input the meaning of life", placeholder: "The answer isn't what you expect", disabled: true }} {template}/>
<Story name="Required" args={{ label: "Please input the meaning of life", placeholder: "The answer isn't what you expect", required: true }} {template}/>
<Story name="Password Input" args={{ label: "Enter your password", required: true, password: true }} {template}/>
<Story name="Character Count" args={{ label: "Enter the entire works of shakespere", counter: true}} {template}/>