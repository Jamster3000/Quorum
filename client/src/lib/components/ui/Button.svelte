<script lang="ts">
  export let href: string | undefined = undefined;
  export let variant: 'primary' | 'secondary' = 'primary';
  export let type: 'button' | 'submit' = 'button';
  export let disabled: boolean = false;
  export let fontSize: 'xsmall' | 'small' | 'medium' | 'large' | 'xlarge' = 'medium';
</script>

{#if href}
  <a {href} class="btn {variant}  {fontSize}">
    <slot />
  </a>
{:else}
  <button {type} {disabled} class="btn {variant}" on:click>
    <slot />
  </button>
{/if}

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 10px 24px;
    border-radius: 999px;
    font-weight: 700;
    font-family: var(--font-primary);
    text-decoration: none;
    border: none;
    cursor: pointer;
    position: relative;
    overflow: hidden;
    transition: transform 0.15s, box-shadow 0.15s;
  }

  .btn::before {
    content: '';
    position: absolute;
    inset: 0;
    background: white;
    opacity: 0;
    transition: opacity 0.15s;
  }

  .btn:hover::before { 
      opacity: 0.08; 
  }

  .btn:active {
      transform: scale(0.97); 
  }

  .primary {
    background: var(--primary-colour);
    color: var(--background-colour);
    box-shadow: 0 0 16px color-mix(in srgb, var(--primary-colour) 40%, transparent);
  }

  .primary:hover {
    box-shadow: 0 0 24px color-mix(in srgb, var(--primary-colour) 60%, transparent);
  }

  .secondary {
    background: transparent;
    color: var(--secondary-colour);
    border: 1.5px solid var(--secondary-colour);
    box-shadow: 0 0 16px color-mix(in srgb, var(--secondary-colour) 20%, transparent);
  }

  .secondary:hover {
    background: color-mix(in srgb, var(--secondary-colour) 10%, transparent);
    box-shadow: 0 0 24px color-mix(in srgb, var(--secondary-colour) 35%, transparent);
  }

  .btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    box-shadow: none;
    transform: none;
  }

  .xsmall {
      font-size: var(--font-xsmall);
  }

  .small {
      font-size: var(--font-small);
  }

  .medium {
      font-size: var(--font-medium);
  }

  .large {
      font-size: var(--font-large);
  }

  .xlarge {
      font-size: var(--font-xlarge);
  }
</style>