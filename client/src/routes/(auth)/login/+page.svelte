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
  import { setAuthStoreValues } from '$lib/stores/authStore';
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
  let copyButtonText = 'Copy';

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

  async function copyBackupCodes() {
    const codesAsText = backupCodes.join(', ');

    try {
      await navigator.clipboard.writeText(codesAsText);

      copyButtonText = 'Copied!';

      setTimeout(() => {
        copyButtonText = 'Copy';
      }, 2000);
    } catch (error) {
      console.error('Failed to copy backup codes:', error);
    }
  }

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
      await setAuthStoreValues(result.access_token, result.refresh_token, result.user_id, result.username);
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

  function downloadBackupCodes() {
    if (backupCodes.length === 0) {
      return;
    }

    const csvRows = [
      'backup_code',
      ...backupCodes.map((code) => `"${code.replace(/"/g, '""')}"`)
    ];

    const csvContent = csvRows.join('\n');

    const csvFile = new Blob(
      [csvContent],
      { type: 'text/csv;charset=utf-8' }
    );

    const downloadUrl = URL.createObjectURL(csvFile);

    const downloadLink = document.createElement('a');
    downloadLink.href = downloadUrl;
    downloadLink.download = 'quorum-backup-codes.csv';

    document.body.appendChild(downloadLink);
    downloadLink.click();
    downloadLink.remove();

    URL.revokeObjectURL(downloadUrl);
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
    <h2 id="backup-codes-title">Save your Backup codes</h2>

    <p id="backup-codes-description">
      These codes can be used to recover your account.
      Store them somewhere safe because they will not be shown again.
    </p>
    <div class="black-box">
  <div class="black-box-actions">
    <button
      class="copy-backup-codes-button"
      type="button"
      on:click={copyBackupCodes}
    >
      {copyButtonText}
    </button>
  </div>

  <div class="backup-codes">
    {#each backupCodes as code}
      <code>{code}</code>
    {/each}
  </div>
</div>

    <div class="backup-popup-actions">
      <button
        class="download-backup-codes-button"
        type="button"
        on:click={downloadBackupCodes}
      >
        Download backup codes
      </button>

      <button
        class="backup-popup-button"
        type="button"
        on:click={closeBackupCodesPopup}
      >
        I have saved these codes
      </button>
    </div>
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

  .black-box {
    width: 100%;
    margin-top: 3rem;
    padding: 1rem 2rem 2rem;
    background-color: #242526;
  }

  .black-box-actions {
    width: 100%;
    display: flex;
    justify-content: flex-end;
    margin-bottom: 1.25rem;
  }

  .backup-popup-actions {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    margin-top: 1.25rem;
  }

  .download-backup-codes-button {
    padding: 0.75rem 1.25rem;
    border: 1px solid var(--primary-colour);
    border-radius: 8px;
    background: transparent;
    color: var(--text-colour);
    font: inherit;
    font-weight: 700;
    cursor: pointer;
  }

  .download-backup-codes-button:hover {
    background: color-mix(
      in srgb,
      var(--primary-colour) 15%,
      transparent
    );
  }

  .download-backup-codes-button:active {
    transform: translateY(1px);
  }

  .copy-backup-codes-button {
    padding: 0.5rem 0.9rem;
    border: 1px solid color-mix(
      in srgb,
      var(--text-colour) 20%,
      transparent
    );
    border-radius: 6px;
    background: color-mix(
      in srgb,
      var(--text-colour) 8%,
      transparent
    );
    color: var(--text-colour);
    font: inherit;
    font-weight: 600;
    cursor: pointer;
  }

  .copy-backup-codes-button:hover {
    background: color-mix(
      in srgb,
      var(--text-colour) 14%,
      transparent
    );
  }

  .copy-backup-codes-button:active {
    transform: translateY(1px);
  }

  .backup-popup {
    width: min(900px, 100%);
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
    grid-template-columns: repeat(2, max-content);
    justify-content: center;
    row-gap: 1.5rem;
    column-gap: 6rem;
    margin: 0;
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
