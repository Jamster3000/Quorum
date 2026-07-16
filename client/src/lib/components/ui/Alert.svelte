<script lang="ts">
  import { IconAlertCircle, IconCheck, IconInfoCircle } from '@tabler/icons-svelte-runes';
  import { fade, slide } from 'svelte/transition';

  export let type: 'error' | 'success' | 'info' | 'warning' = 'error';
  export let message: string = '';
  export let show: boolean = false;

  $: alertClass = `alert ${type}`;
</script>

{#if show}
  <div
    class={alertClass}
    role="alert"
    aria-live="assertive"
    in:slide={{ duration: 300 }}
    out:fade={{ duration: 150 }}
  >
    <span class="alert-icon">
      {#if type === 'error'}
        <IconAlertCircle size={18} />
      {:else if type === 'success'}
        <IconCheck size={18} />
      {:else if type === 'info'}
        <IconInfoCircle size={18} />
      {:else if type === 'warning'}
        <IconAlertCircle size={18} />
      {/if}
    </span>
    <span class="alert-message">{message}</span>
  </div>
{/if}

<style>
  .alert {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    margin: 0.5rem 0;
    border-radius: 8px;
    font-family: var(--font-primary);
    font-size: var(--font-small);
    width: 100%;
    max-width: 600px;
    border: 1px solid;
  }

  .alert.error {
    color: var(--alert-error-text);
    background-color: var(--alert-error-bg);
    border-color: var(--alert-error-border);
  }

  .alert.success {
    color: var(--alert-success-text);
    background-color: var(--alert-success-bg);
    border-color: var(--alert-success-border);
  }

  .alert.info {
    color: var(--alert-info-text);
    background-color: var(--alert-info-bg);
    border-color: var(--alert-info-border);
  }

  .alert.warning {
    color: var(--alert-warning-text);
    background-color: var(--alert-warning-bg);
    border-color: var(--alert-warning-border);
  }

  .alert-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .alert-message {
    line-height: 1.5;
  }
</style>