<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import { userEvent, within, expect } from 'storybook/test';
    import Nav from '../Nav.svelte';
    import Card from '../Card.svelte';

    const { Story } = defineMeta({
        title: 'UI/Nav',
        tags: ['autodocs'],
        component: Nav,
    });
</script>

{#snippet template(args)}
    <Card height="100%">
        <Nav {...args}/>
    </Card>
{/snippet}

<Story name="Links" args={{ links: [{ label: 'Home', href: '/' },{ label: 'Over the rainbow', href: '/rainbow' }] }} {template}/>

<Story 
  name="Navigate" 
  args={{ 
    links: [
      { href: '/home', label: 'Home' },
      { href: '/about', label: 'About' }
    ]
  }}
  play={async ({ canvas, userEvent }) => {
    const link = canvas.getByTestId('nav-link-home');
    expect(link).toHaveAttribute('href', '/home');
  }}
/>