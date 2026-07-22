 <script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import Alert from '$lib/components/ui/Alert.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Checkbox from '$lib/components/ui/Checkbox.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { IconHexagonFilled, IconArrowLeft } from '@tabler/icons-svelte-runes';

  interface SignupSuccess {
    message: string;
  }

  let username = '';
  let email = '';
  let password = '';
  let confirmPassword = '';
  let agreedToTerms = false;
  let formError = '';
  let loading = false;
  let errorMessage = '';
  let showAlert = false;
  let alertType = 'error';

  let errors = {
    username: '',
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
      const result = await invoke<SignupSuccess>('signup', {
        payload: {
          username,
          email: email || null,
          password,
          confirm_password: confirmPassword,
        }
      });

      handleSignupSuccess(result.message);
      goto("/login?created=1");
    } catch (e) {
      handleSignupFailure(e as string);
    } finally {
      loading = false;
    }
  }

  function handleSignupFailure(message: string) {
    errorMessage = message;
    alertType = 'error';
    showAlert = true;
  }

  function handleSignupSuccess(message: string) {
    errorMessage = message;
    alertType = 'success';
    showAlert = true;
  }
</script>

<main class="page">
   <a href="/" class="back-link">
     <IconArrowLeft size={20} />
     Back to home
   </a>

  <div class="signup-wrap">
    <div class="signup-header">
      <IconHexagonFilled size={32} color="var(--primary-colour)" />
      <h1>Create your account</h1>
    </div>

    <Card padding="xlarge" center={false}>
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
          label="Username"
          placeholder="e.g. dragonslayer99"
          bind:value={username}
          error={errors.username}
          counter={true}
          maxLength={18}
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
    position: relative;
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
    font-size: var(--font-xlarge);
    font-weight: 700;
    color: var(--text-colour);
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 2rem;
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
    font-size: var(--font-xsmall);
    color: var(--text-colour);
  }

.back-link {
  position: absolute;
  display: inline-flex;
  align-items: center;
  top: 2rem;
  left: 2rem;
  gap: 6px;
  font-size: var(--font-small);
  font-weight: 700;
  color: var(--text-colour);
  text-decoration: none;
  transition: color 0.15s;
}

.back-link:hover {
    color: var(--text-colour);
}
</style>
