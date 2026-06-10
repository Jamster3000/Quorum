<script>
  export let type = "error";
  export let message = "";
  export let show = false;

  // Compute the full class string
  $: alertClass = `alert ${type}`;
</script>

{#if show}
  <div
    class={alertClass}
    role="alert"
    aria-live="assertive"
  >
    <span class="alert-icon">{type === "error" ? "!" : type === "success" ? "✓" : "i"}</span>
    <span class="alert-message">{message}</span>
  </div>
{/if}

<style>
  .alert {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    margin: 0.5rem 0;
    border-radius: 4px;
    font-family: inherit;
    font-size: 0.875rem;
    width: fit-content;
  }

  .alert.error {
    color: var(--alert-error-text);
    background-color: var(--alert-error-bg);
    border: 1px solid var(--alert-error-border);
  }

  .alert.success {
    color: var(--alert-success-text);
    background-color: var(--alert-success-bg);
    border: 1px solid var(--alert-success-border);
  }

  .alert.info {
    color: var(--alert-info-text);
    background-color: var(--alert-info-bg);
    border: 1px solid var(--alert-info-border);
  }

  .alert.warning {
    color: var(--alert-warning-text);
    background-color: var(--alert-warning-bg);
    border: 1px solid var(--alert-warning-border);
  }

  .alert-icon {
    font-weight: bold;
    font-size: 1.1rem;
  }
</style>