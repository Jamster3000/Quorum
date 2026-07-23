<script lang="ts">
  export let isOpen = false;
  export let fullscreen = false;
  export let onClose: (() => void) | undefined = undefined;
  export let closeOnBackdrop = false;

  function handleBackdropClick(e: MouseEvent) {
    if (closeOnBackdrop && e.target === e.currentTarget) {
      isOpen = false;
      onClose?.();
    }
  }

  function handleClose() {
    isOpen = false;
    onClose?.();
  }
</script>

{#if isOpen}
  <div
      class="dialog-backdrop"
      class:fullscreen
      on:click={handleBackdropClick}
      role="presentation"
      tabindex="-1"
    >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="dialog-container"
      on:click={handleBackdropClick}
    >
      <slot />
    </div>
  </div>
{/if}

<style>
  .dialog-backdrop {
    position: fixed;
    left: 0;
    right: 0;
    top: calc(var(--titlebar-height, 0px) + var(--header-height, 0px));
    bottom: var(--footer-height, 0px);
    background: rgba(0, 0, 0, 0.7);
    z-index: 1000;
  }

  .dialog-backdrop.fullscreen {
    top: var(--titlebar-height, 0px);
    bottom: 0;
  }

  .dialog-container {
    width: 100%;
    height: 100%;
    position: relative;
    background: transparent;
    padding: var(--page-padding);
    margin: 0;
    overflow: visible;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>