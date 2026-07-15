<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { IconMinus, IconSquares, IconX, IconHexagonFilled } from '@tabler/icons-svelte-runes';

  const appWindow = getCurrentWindow();
</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="titlebar-title" data-tauri-drag-region>
    <IconHexagonFilled size={18} color="var(--text-colour)" />  
    Quorum
  </div>

  <div class="titlebar-controls">
    <button class="minimise" type="button" on:click={() => appWindow.minimize()}>
        <IconMinus size={16} color="white" />
    </button>

    <button class="maximise" type="button" on:click={() => appWindow.toggleMaximize()}>
        <IconSquares size={16} color="white" />
    </button>

    <button class="close" type="button" on:click={() => appWindow.close()}>
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
    padding-left: 8px;
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