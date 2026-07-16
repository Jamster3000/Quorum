<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import Alert from '$lib/components/ui/Alert.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Checkbox from '$lib/components/ui/Checkbox.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { IconHexagonFilled, IconArrowLeft } from '@tabler/icons-svelte-runes';
  import { setTokens } from '$lib/stores/authStore';

  let username_or_email = '';
  let password = '';
  let confirmPassword = '';
  let agreedToTerms = false;
  let formError = '';
  let loading = false;
  let errorMessage = '';
  let showAlert = false;
  let alertType = 'error';

  $: fromSignup = $page.url.searchParams.get('created') === '1';
  $: fromPage = $page.url.searchParams.get('from') || null;

  let errors = {
    username_or_email: '',
    email: '',
    password: '',
    confirmPassword: '',
    terms: '',
    form: '',
  };

  async function handleSubmit() {
    formError = '';
    loading = true;

    try {
      const result = await invoke<{ success: boolean; message: string; access_token: string, refresh_token: string, user_id: string, username: string }>('login', {
        payload: {
          username_or_email: username_or_email,
          password: password,
        }
      });

      if (!result.success) {
        handleLoginFailure(result.message);
      } else {
        handleLoginSuccess(result.message);
        await setTokens(result.access_token, result.refresh_token, result.user_id, result.username)
        goto("/");
      }
    } catch (e) {
      formError = 'Something went wrong. Please try again.';
    } finally {
      loading = false;
    }
  }

  function handleLoginFailure(message: string) {
    errorMessage = message;
    alertType = 'error';
    showAlert = true;
  }

  function handleLoginSuccess(message: string) {
    errorMessage = message;
    alertType = 'success';
    showAlert = true;
  }
</script>

<main class="page">
  <div class="login-wrap">

      {#if fromSignup}
      <Alert
        type="success"
        message="Account created! Please log in."
        show={true}
      />
    {:else if fromPage}
      <Alert
        type="warning"
        message="Sorry, you need to be logged in to access that page."
        show={true}
      />
    {/if}

    <a href="/" class="back-link">
      <IconArrowLeft size={16} />
      Back to home
    </a>

    <div class="login-header">
      <IconHexagonFilled size={32} color="var(--primary-colour)" />
      <h1>Log in to your account</h1>
    </div>

    <Card padding="large">
      <form class="form" on:submit|preventDefault={handleSubmit}>

        {#if errors.form}
          <div class="form-error">{errors.form}</div>
        {/if}

        <Alert
            type={alertType}
            message={errorMessage}
            show={showAlert}
        />

        <Input
          label="Username or Email"
          placeholder="e.g. dragonslayer99"
          bind:value={username_or_email}
          error={errors.username_or_email}
          required
        />

        <Input
          label="Password"
          placeholder="At least 8 characters"
          bind:value={password}
          error={errors.password}
          password
          required
        />

        <Button variant="primary" type="submit" fontSize="medium">Log in</Button>

        <p class="signup-link">Don't have an account? <a href="/login">Sign up</a></p>
      </form>
    </Card>

  </div>
</main>

<style>
  .page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
  }

  .login-wrap {
    width: 100%;
    max-width: 560px;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .login-header {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    text-align: center;
  }

  .login-header h1 {
    font-size: 24px;
    font-weight: 700;
    color: var(--text-colour);
  }

  .signup-link {
    text-align: center;
    font-size: 13px;
    color: var(--text-colour);
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .form-error {
    background: color-mix(in srgb, var(--error-colour) 12%, transparent);
    border: 1px solid color-mix(in srgb, var(--error-colour) 30%, transparent);
    color: var(--error-colour);
    font-size: 13px;
    font-weight: 600;
    padding: 10px 14px;
    border-radius: 10px;
  }

.back-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  font-weight: 700;
  color: var(--text-colour);
  text-decoration: none;
  transition: color 0.15s;
}

.back-link:hover {
    color: var(--text-colour);
}
</style>