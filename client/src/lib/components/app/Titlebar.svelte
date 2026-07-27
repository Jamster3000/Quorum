<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount } from 'svelte';
  import { IconMinus, IconSquares, IconX, IconHexagonFilled, IconCircleArrowLeft, IconCircleArrowRight  } from '@tabler/icons-svelte-runes';
  import Button from '$lib/components/ui/Button.svelte';

  let appWindow: any;

  onMount(async () => {
    appWindow = await getCurrentWindow();
  });
</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="titlebar-title" data-tauri-drag-region>
    <Button variant="transparent" iconOnly={true} on:click={() => window.history.back()} ariaLabel="Back Button">
      <IconCircleArrowLeft size={24} color="var(--text-colour)" />
    </Button>

    <Button variant="transparent" iconOnly={true} on:click={() => window.history.forward()} ariaLabel="Forward Button">
      <IconCircleArrowRight size={24} color="var(--text-colour)" />
    </Button>
    
    <p class="title-text">Quorum</p>
    <IconHexagonFilled size={18} color="var(--text-colour)" />  
  </div>

  <div class="titlebar-controls">
    <button class="minimise" type="button" on:click={() => appWindow.minimize()} aria-label="Minimise Button">
        <IconMinus size={16} color="white" />
    </button>

    <button class="maximise" type="button" on:click={() => appWindow.toggleMaximize()} aria-label="Maximise Button">
        <IconSquares size={16} color="white" />
    </button>

    <button class="close" type="button" aria-label="Close Button" on:click={() => appWindow.close()}>
        <IconX size={16} color="white" />
    </button>
  </div>
</div>

<style>
.titlebar {
  height: var(--titlebar-height);
  display: flex;
  align-items: center;
  justify-content: space-between;
  user-select: none;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  background: var(--primary-colour);
  color: var(--text-colour);
  z-index: 100000;
}

.titlebar-title {
    display: flex;
    align-items: center;
    gap: 6px;
    padding-left: 4px;
}

.title-text {
    padding-left: 14px;
}

.titlebar-controls {
  display: flex;
  gap: 4px;
  align-self: stretch;
}

.titlebar-controls button {
  width: 46px;
  height: 100%;
  border: 0;
  background: transparent;
  color: inherit;
  cursor: pointer;
  transition: background 0.15s;
}

.titlebar-controls button:hover {
  background: rgba(255, 255, 255, 0.2);
}

.titlebar-controls button.close:hover {
  background: var(--alert-error-border);
}
</style>