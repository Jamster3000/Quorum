<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import Alert from '../Alert.svelte';

    const { Story } = defineMeta({
        title: 'UI/Alert',
        component: Alert,
    });
</script>

{#snippet template(args)}
    <Alert {...args}/>
{/snippet}

<Story name="Error" args={{ type: "error", show: true, message: "This is an Alert component"}} {template}/>
<Story name="Warning" args={{ type: "warning", show: true, message: "This is an Alert component"}} {template}/>
<Story name="Success" args={{ type: "success", show: true, message: "This is an Alert component"}} {template}/>
<Story name="Info" args={{ type: "info", show: true, message: "This is an Alert component"}} {template}/>
