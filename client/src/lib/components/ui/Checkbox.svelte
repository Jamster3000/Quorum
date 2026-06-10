<script lang="ts">
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
      <svg class="tick" viewBox="0 0 12 10" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M1 5L4.5 8.5L11 1" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </div>
    <span class="label-text">
      <slot>{label}</slot>
    </span>
  </label>

  {#if error}
    <span class="error-text">{error}</span>
  {/if}
</div>

<style>
  .checkbox-field {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  input[type="checkbox"] {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .checkbox-row {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    user-select: none;
  }

  .checkbox-row.disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .box {
    width: 20px;
    height: 20px;
    min-width: 20px;
    border-radius: 6px;
    border: 1.5px solid var(--border-colour);
    background: var(--background-colour);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.2s, border-color 0.2s, box-shadow 0.2s;
    position: relative;
    overflow: hidden;
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

  .tick {
    width: 12px;
    height: 10px;
    color: var(--background-colour);
    position: relative;
    z-index: 1;
    stroke-dasharray: 20;
    stroke-dashoffset: 20;
    transition: stroke-dashoffset 0.2s ease 0.05s;
  }

  .box.checked .tick {
    stroke-dashoffset: 0;
  }

  input[type="checkbox"]:focus-visible + .box {
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--primary-colour) 25%, transparent);
    border-color: var(--primary-colour);
  }

  .label-text {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-muted-colour);
    line-height: 1.5;
  }

  .label-text :global(a) {
    color: var(--primary-colour);
    text-decoration: none;
    font-weight: 700;
  }

  .label-text :global(a:hover) {
    text-decoration: underline;
  }

  .error-text {
    font-size: 12px;
    font-weight: 600;
    color: var(--error-colour);
    padding-left: 30px;
  }
</style>