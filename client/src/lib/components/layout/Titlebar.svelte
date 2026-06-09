<script>
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="titlebar-title" data-tauri-drag-region>quorum</div>

  <div class="titlebar-controls">
    <button type="button" on:click={() => appWindow.minimize()}>—</button>
    <button type="button" on:click={() => appWindow.toggleMaximize()}>□</button>
    <button type="button" on:click={() => appWindow.close()}>✕</button>
  </div>
</div>

<style>
.titlebar {
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 8px;
  user-select: none;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  background: #1e1e1e;
  color: white;
}

.titlebar-controls {
  display: flex;
  gap: 4px;
}

.titlebar-controls button {
  width: 32px;
  height: 24px;
  border: 0;
  background: transparent;
  color: inherit;
  cursor: pointer;
}

</style>