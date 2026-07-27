<script module>
    import { defineMeta } from '@storybook/addon-svelte-csf';
    import { userEvent, expect, within } from 'storybook/test';
    import Input from '../Input.svelte';
    import Card from '../Card.svelte'

    const { Story } = defineMeta({
        title: 'UI/Input',
        tags: ['autodocs'],
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

<Story 
  name="Max Character" 
  args={{ 
      type: 'text', 
      max: 5,
      label: "Max character input",
  }}
  play={async ({ canvas, userEvent }) => {
    const input = canvas.getByTestId('input-field');
    await userEvent.clear(input);
    await userEvent.type(input, 'hello');
    expect(input.value).toBe('hello');
  }}
/>

<Story 
  name="Password Toggle" 
  args={{ 
      type: 'password', 
      password: true,
      label: "Password input with toggle",
  }}
  play={async ({ canvas, userEvent }) => {
    const input = canvas.getByTestId('input-field');
    const toggle = canvas.getByTestId('password-toggle');
    
    expect(input).toHaveAttribute('type', 'password');
    await userEvent.click(toggle);
    expect(input).toHaveAttribute('type', 'text');
  }}
/>