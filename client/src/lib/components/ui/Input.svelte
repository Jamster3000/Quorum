<script lang="ts">
  import { IconEye, IconEyeOff } from '@tabler/icons-svelte-runes';

  export let label: string = '';
  export let type: 'text' | 'password' | 'email' | 'hidden' | 'number' | 'search' | 'tel' | 'url' = 'text';
  export let placeholder: string = '';
  export let value: string = '';
  export let error: string = '';
  export let helper: string = '';
  export let disabled: boolean = false;
  export let required: boolean = false;
  export let password: boolean = false;
  export let counter: boolean = false;
  export let maxLength: number | undefined = undefined;
  export let id: string = crypto.randomUUID();

  let showPassword = false;

  const validTypes = ['text', 'password', 'email', 'hidden', 'number', 'search', 'tel', 'url'];
  $: safeType = validTypes.includes(type) ? type : 'text';
  $: inputType = password ? (showPassword ? 'text' : 'password') : safeType;
</script>

<div class="field">
  {#if label}
    <label for={id}>
      {label}
      {#if required}<span class="required">*</span>{/if}
    </label>
  {/if}

  <div class="input-wrap" class:error={!!error} class:disabled>
    <input
      {id}
      type={inputType}
      {placeholder}
      {disabled}
      {required}
      maxlength={maxLength}
      bind:value
      on:input
      on:blur
      on:focus
    />
    {#if password}
      <button
        type="button"
        class="eye-toggle"
        on:click={() => showPassword = !showPassword}
        aria-label={showPassword ? 'Hide password' : 'Show password'}
        {disabled}
      >
        {#if showPassword}
          <IconEyeOff size={16} />
        {:else}
          <IconEye size={16} />
        {/if}
      </button>
    {/if}
  </div>

  <div class="counter">
    {#if counter}
      <span class="counter">
        {value.length}{maxLength !== undefined ? ` / ${maxLength}` : ''}
      </span>
    {/if}
  </div>

  {#if error}
    <span class="helper error-text">{error}</span>
  {:else if helper}
    <span class="helper">{helper}</span>
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  label {
    font-size: var(--font-medium);
    font-weight: 400;
    color: var(--text-colour);
  }

  .required {
    color: var(--alert-error-text);
    margin-left: 2px;
  }

  .input-wrap {
    display: flex;
    align-items: center;
    background: var(--background-colour);
    border: 1.5px solid var(--border-colour);
    border-radius: 8px;
    transition: border-color 0.15s, box-shadow 0.15s, background 0.15s;
  }

  .input-wrap:focus-within {
    border-color: var(--primary-colour);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--primary-colour) 15%, transparent);
  }

  .input-wrap.error {
    border-color: var(--alert-error-text);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--alert-error-text) 15%, transparent);
  }

  .input-wrap.disabled {
    background: color-mix(in srgb, var(--card-colour) 50%, transparent);
    opacity: 0.6;
    cursor: not-allowed;
  }

  input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    padding: 10px 14px;
    font-size: var(--font-small);
    font-family: var(--font-primary);
    color: var(--text-colour);
    width: 100%;
  }

  input::placeholder {
    color: var(--text-colour);
    opacity: 0.6;
  }

  input:disabled {
    cursor: not-allowed;
  }

  .eye-toggle {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-colour);
    transition: opacity 0.15s;
  }

  .eye-toggle:hover:not(:disabled) {
    opacity: 0.7;
  }

  .eye-toggle:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .eye-toggle:focus-visible {
    outline: 2px solid var(--primary-colour);
    outline-offset: 2px;
  }

  .helper {
    font-size: var(--font-small);
    font-weight: 400;
    color: var(--text-colour);
  }

  .error-text {
    color: var(--alert-error-bg-text);
  }

  .counter {
    color: var(--text-colour);
    font-size: var(--font-small);
    opacity: 0.8;
  }
</style>
