<script lang="ts">
  import { IconCheck } from '@tabler/icons-svelte-runes';

  export let label: string = '';
  export let checked: boolean = false;
  export let disabled: boolean = false;
  export let error: string = '';
  export let id: string = crypto.randomUUID();
</script>

<div class="checkbox-field">
  <label for={id} class="checkbox-row" class:disabled>
    <input
      {id}
      {disabled}
      type="checkbox"
      bind:checked
      on:change
    />
    <div class="box" class:checked>
      {#if checked}
        <IconCheck size={18} color="white" style="position: relative; z-index: 1;" />
      {/if}
    </div>
    <span class="label-text">
      {#if label}{label}{/if}
      <slot />
    </span>
  </label>

  {#if error}
    <span class="error-text">{error}</span>
  {/if}
</div>

<style>
  .checkbox-field {
    display: block;
  }

  input[type="checkbox"] {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .checkbox-row {
    display: block;
    cursor: pointer;
    user-select: none;
    margin-bottom: 6px;
  }

  .checkbox-row.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .box {
    width: 20px;
    height: 20px;
    border-radius: 6px;
    border: 1.5px solid var(--border-colour);
    background: var(--background-colour);
    display: inline-block;
    vertical-align: middle;
    text-align: center;
    line-height: 20px;
    transition: background 0.2s, border-color 0.2s, box-shadow 0.2s;
    position: relative;
    overflow: hidden;
    box-sizing: border-box;
  }

  .box::before {
    content: '';
    position: absolute;
    inset: 0;
    background: var(--primary-colour);
    transform: scale(0);
    border-radius: 4px;
    transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .box.checked {
    border-color: var(--primary-colour);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--primary-colour) 15%, transparent);
  }

  .box.checked::before {
    transform: scale(1);
  }

  input[type="checkbox"]:focus-visible + .box {
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--primary-colour) 25%, transparent);
    border-color: var(--primary-colour);
  }

  .label-text {
    display: inline-block;
    vertical-align: middle;
    margin-left: 6px;
    font-size: var(--font-small);
    font-weight: 600;
    color: var(--text-colour);
    line-height: 1.5;
  }

  .error-text {
    display: block;
    font-size: 12px;
    font-weight: 600;
    color: var(--alert-error-text);
  }
</style>