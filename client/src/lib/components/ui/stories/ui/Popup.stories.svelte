<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import Popup from '../../Popup.svelte';
    import Card from '../../Card.svelte';

    const { Story } = defineMeta({
        title: 'UI/Popup',
        component: Popup,
    });
</script>

{#snippet template(args)}
    <Popup {...args}>
        <Card>
            <p>This is a popup with a card inside</p>
        </Card>
    </Popup>
{/snippet}

<Story name="Popup" args={{ isOpen: true }} {template}/>
<Story name="Fullscreen" args={{ isOpen: true, fullscreen: true }} {template}/>
<Story name="Close on backdrop click" args={{ isOpen: true, closeOnBackdrop: true }} {template}/>
