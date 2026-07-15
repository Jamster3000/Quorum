<script lang="ts">
  import { IconEye, IconEyeOff } from '@tabler/icons-svelte-runes';

  export let label: string = '';
  export let type: string = 'text';
  export let placeholder: string = '';
  export let value: string = '';
  export let error: string = '';
  export let helper: string = '';
  export let disabled: boolean = false;
  export let required: boolean = false;
  export let password: boolean = false;
  export let id: string = crypto.randomUUID();

  let showPassword = false;

  $: inputType = password ? (showPassword ? 'text' : 'password') : type;
</script>

<div class="field">
  {#if label}
    <label for={id}>
      {label}
      {#if required}<span class="required">*</span>{/if}
    </label>
  {/if}

  <div class="input-wrap" class:error={!!error}>
    <input
      {id}
      type={inputType}
      {placeholder}
      {disabled}
      {required}
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
      >
        {#if showPassword}
          <IconEyeOff size={16} color="var(--text-colour)" />
        {:else}
          <IconEye size={16} color="var(--text-colour)" />
        {/if}
      </button>
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
    font-size: 13px;
    font-weight: 700;
    color: var(--text-colour);
  }

  .required {
    color: var(--error-colour);
    margin-left: 2px;
  }

  .input-wrap {
    display: flex;
    align-items: center;
    background: var(--background-colour);
    border: 1.5px solid var(--border-colour);
    border-radius: 10px;
    transition: border-color 0.15s, box-shadow 0.15s;
  }

  .input-wrap:focus-within {
    border-color: var(--primary-colour);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--primary-colour) 15%, transparent);
  }

  .input-wrap.error {
    border-color: var(--error-colour);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--error-colour) 15%, transparent);
  }

  input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    padding: 10px 14px;
    font-size: 14px;
    font-family: var(--font-primary);
    color: var(--text-colour);
    width: 100%;
  }

  input::placeholder {
    color: var(--text-colour);
    opacity: 0.6;
  }

  input:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .eye-toggle {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0 12px;
    display: flex;
    align-items: center;
    height: 100%;
  }

  .eye-toggle:hover :global(svg) {
    stroke: var(--text-colour);
  }

  .helper {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-colour);
  }

  .error-text {
    color: var(--error-colour);
  }
</style>