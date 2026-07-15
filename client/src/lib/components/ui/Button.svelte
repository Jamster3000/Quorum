<script lang="ts">
  export let href: string | undefined = undefined;
  export let variant: 'primary' | 'secondary' = 'primary';
  export let type: 'button' | 'submit' = 'button';
  export let disabled: boolean = false;
  export let fontSize: 'xsmall' | 'small' | 'medium' | 'large' | 'xlarge' = 'medium';
</script>

{#if href}
  <a {href} class="btn {variant} {fontSize}" class:disabled>
    <span class="btn-text"><slot /></span>
  </a>
{:else}
  <button {type} {disabled} class="btn {variant} {fontSize}" on:click>
    <span class="btn-text"><slot /></span>
  </button>
{/if}

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 10px 20px;
    border-radius: 6px;
    font-weight: 600;
    font-family: var(--font-primary);
    text-decoration: none;
    border: none;
    cursor: pointer;
    transition:
      background-color 0.15s ease,
      border-color 0.15s ease,
      box-shadow 0.15s ease,
      transform 0.15s ease;
    will-change: box-shadow, transform;
  }

  .btn-text {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    transition: transform 0.2s ease;
  }

  .btn:hover:not(.disabled):not(:disabled) .btn-text {
    transform: scale(1.05);
  }

  .btn:active:not(.disabled):not(:disabled) .btn-text {
    transform: scale(0.95);
  }

  .primary {
    background: var(--primary-colour);
    color: #ffffff;
    box-shadow: 
      0 1px 3px rgba(0, 0, 0, 0.15),
      0 3px 6px rgba(0, 0, 0, 0.1),
      inset 0 -1px 2px rgba(0, 0, 0, 0.2);
  }

  .primary:hover:not(.disabled):not(:disabled) {
    background: color-mix(in srgb, var(--primary-colour) 90%, black);
    box-shadow:
      0 2px 5px rgba(0, 0, 0, 0.15),
      0 5px 10px rgba(0, 0, 0, 0.12),
      inset 0 -1px 2px rgba(0, 0, 0, 0.2);
    transform: translateY(-1px);
  }

  .primary:active:not(.disabled):not(:disabled) {
    box-shadow: inset 0 1px 4px rgba(0, 0, 0, 0.3);
    transform: translateY(0);
  }

  .secondary {
    background: transparent;
    color: #20b2aa;
    border: 1.5px solid #20b2aa;
    box-shadow: none;
  }

  .secondary:hover:not(.disabled):not(:disabled) {
    background: color-mix(in srgb, #20b2aa 10%, transparent);
    border-color: #20b2aa;
    transform: translateY(-1px);
  }

  .secondary:active:not(.disabled):not(:disabled) {
    background: color-mix(in srgb, #20b2aa 15%, transparent);
    transform: translateY(0);
  }

  .btn:focus-visible {
    outline: 2px solid var(--primary-colour);
    outline-offset: 2px;
  }

  .btn:disabled,
  .btn.disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none !important;
  }

  .xsmall { font-size: var(--font-xsmall); }
  .small { font-size: var(--font-small); }
  .medium { font-size: var(--font-medium); }
  .large { font-size: var(--font-large); }
  .xlarge { font-size: var(--font-xlarge); }
</style>