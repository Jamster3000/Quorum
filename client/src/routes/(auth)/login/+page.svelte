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
  import { onMount } from 'svelte';
  import Dialog from '$lib/components/ui/Popup.svelte';


  interface AuthSuccess {
    access_token: string;
    refresh_token: string;
    user_id: string;
    username: string;
  }

  let username_or_email = '';
  let password = '';
  let confirmPassword = '';
  let agreedToTerms = false;
  let formError = '';
  let loading = false;
  let errorMessage = '';
  let showAlert = false;
  let alertType = 'error';
  let showNoEmailPopup = false;
  let backupCodes: string[] = [];

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

  onMount(() => {
  const accountWasCreated =
    $page.url.searchParams.get('created') === '1';

  const accountHasNoEmail =
    $page.url.searchParams.get('noEmail') === '1';

  console.log('Login accountWasCreated:', accountWasCreated);
  console.log('Login accountHasNoEmail:', accountHasNoEmail);

  if (!accountWasCreated || !accountHasNoEmail) {
    return;
  }

  const storedCodes =
    sessionStorage.getItem('newAccountBackupCodes');

  console.log('Login storedCodes:', storedCodes);

  if (!storedCodes) {
    return;
  }

  try {
    const parsedCodes: unknown = JSON.parse(storedCodes);

    console.log('Login parsedCodes:', parsedCodes);

    if (
      Array.isArray(parsedCodes) &&
      parsedCodes.length > 0 &&
      parsedCodes.every((code) => typeof code === 'string')
    ) {
      backupCodes = parsedCodes;
      showNoEmailPopup = true;


      console.log('Popup state:', showNoEmailPopup);
    }
  } catch (error) {
    console.error('Could not parse backup codes:', error);

    backupCodes = [];
    showNoEmailPopup = false;
  }
});

  async function handleSubmit() {
    formError = '';
    loading = true;

    try {
      const result = await invoke<AuthSuccess>('login', {
        payload: {
          username_or_email: username_or_email,
          password: password,
        }
      });

      handleLoginSuccess('Logged in successfully');
      await setTokens(result.access_token, result.refresh_token, result.user_id, result.username);
      goto("/home");
    } catch (e) {
      handleLoginFailure(e as string);
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

  function closeBackupCodesPopup() {
  sessionStorage.removeItem('newAccountBackupCodes');
  backupCodes = [];
  showNoEmailPopup = false;
}
</script>

<main class="page">

  <Dialog
  bind:isOpen={showNoEmailPopup}
  fullscreen={true}
  closeOnBackdrop={false}
>
  <div
    class="backup-popup"
    role="dialog"
    aria-labelledby="backup-codes-title"
    aria-describedby="backup-codes-description"
  >
    <h2 id="backup-codes-title">Save your backup codes</h2>

    <p id="backup-codes-description">
      These codes can be used to recover your account.
      Store them somewhere safe because they will not be shown again.
    </p>

    <div class="backup-codes">
      {#each backupCodes as code}
        <code>{code}</code>
      {/each}
    </div>

    <button
      class="backup-popup-button"
      type="button"
      on:click={closeBackupCodesPopup}
    >
      I have saved these codes
    </button>
  </div>
</Dialog>

  <a href="/" class="back-link">
    <IconArrowLeft size={20} />
      Back to home
  </a>

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

    <div class="login-header">
      <IconHexagonFilled size={32} color="var(--primary-colour)" />
      <h1>Log in to your account</h1>
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
          label="Username or Email"
          placeholder="e.g. dragonslayer99"
          bind:value={username_or_email}
          error={errors.username_or_email}
          counter={true}
          maxLength={18}
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

        <p class="signup-link">Don't have an account? <a href="/signup">Sign up</a></p>
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
    font-size: var(--font-xlarge);
    font-weight: 700;
    color: var(--text-colour);
  }

  .signup-link {
    text-align: center;
    font-size: var(--font-xsmall);
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
}









.backup-popup {
  width: min(480px, 100%);
  max-height: 80vh;
  overflow-y: auto;
  padding: 2rem;
  border-radius: 12px;
  background: var(--card-colour, Canvas);
  color: var(--text-colour, CanvasText);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.35);
  text-align: center;
}

.backup-popup h2 {
  margin: 0 0 0.75rem;
  font-size: var(--font-xlarge);
}

.backup-popup p {
  margin: 0;
  line-height: 1.5;
}

.backup-codes {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.75rem;
  margin: 1.5rem 0;
}

.backup-codes code {
  display: block;
  padding: 0.75rem;
  border: 1px solid color-mix(
    in srgb,
    var(--text-colour) 20%,
    transparent
  );
  border-radius: 8px;
  overflow-wrap: anywhere;
  font-family: monospace;
  text-align: center;
}

.backup-popup-button {
  padding: 0.75rem 1.25rem;
  border: none;
  border-radius: 8px;
  background: var(--primary-colour);
  color: white;
  font: inherit;
  font-weight: 700;
  cursor: pointer;
}

@media (max-width: 500px) {
  .backup-codes {
    grid-template-columns: 1fr;
  }
}

</style>
