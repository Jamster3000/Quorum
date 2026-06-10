<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import Titlebar from '$lib/components/layout/Titlebar.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Checkbox from '$lib/components/ui/Checkbox.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { IconHexagonFilled, IconArrowLeft } from '@tabler/icons-svelte';

  let username = '';
  let email = '';
  let password = '';
  let confirmPassword = '';
  let agreedToTerms = false;
  let formError = '';
  let loading = false;

  let errors = {
    username: '',
    email: '',
    password: '',
    confirmPassword: '',
    terms: '',
    form: '',
  };

  async function handleSubmit() {
    if (!agreedToTerms) {
      formError = 'You must agree to the Terms of Service.';
      return;
    }

    formError = '';
    loading = true;

    try {
      const result = await invoke<{ success: boolean; message: string }>('signup', {
        payload: {
          username,
          email: email || null,
          password,
          confirm_password: confirmPassword,
        }
      });

      if (!result.success) {
        formError = result.message;
      } else {
        goto("/login?ac=t");
      }
    } catch (e) {
      formError = 'Something went wrong. Please try again.';
    } finally {
      loading = false;
    }
  }
</script>

<Titlebar />

<main class="page">
  <div class="signup-wrap">

    <a href="/" class="back-link">
      <IconArrowLeft size={16} />
      Back to home
    </a>

    <div class="signup-header">
      <IconHexagonFilled size={32} color="var(--primary-colour)" />
      <h1>Create your account</h1>
    </div>

    <Card padding="large">
      <form class="form" on:submit|preventDefault={handleSubmit}>

        {#if errors.form}
          <div class="form-error">{errors.form}</div>
        {/if}

        <Input
          label="Username"
          placeholder="e.g. dragonslayer99"
          bind:value={username}
          error={errors.username}
          required
        />

        <Input
          label="Email"
          type="email"
          placeholder="Optional"
          bind:value={email}
          error={errors.email}
          helper="An email address is optional but recommended for account recovery."
        />

        <Input
          label="Password"
          placeholder="At least 8 characters"
          bind:value={password}
          error={errors.password}
          password
          required
        />

        <Input
          label="Confirm password"
          placeholder="Repeat your password"
          bind:value={confirmPassword}
          error={errors.confirmPassword}
          password
          required
        />

        <Checkbox bind:checked={agreedToTerms} error={errors.terms}>
          I agree to the <a href="/terms">Terms of Service</a>
        </Checkbox>

        <Button variant="primary" type="submit" fontSize="medium">Create account</Button>

        <p class="login-link">Already have an account? <a href="/login">Log in</a></p>

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

  .signup-wrap {
    width: 100%;
    max-width: 560px;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .signup-header {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    text-align: center;
  }

  .signup-header h1 {
    font-size: 24px;
    font-weight: 700;
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

  .login-link {
    text-align: center;
    font-size: 13px;
    color: var(--text-muted-colour);
  }

  .login-link a, .form a {
    color: var(--primary-colour);
    text-decoration: none;
    font-weight: 700;
  }

  .login-link a:hover, .form a:hover {
    text-decoration: underline;
  }

.back-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  font-weight: 700;
  color: var(--text-muted-colour);
  text-decoration: none;
  transition: color 0.15s;
}

.back-link:hover {
    color: var(--text-colour);
}
</style>