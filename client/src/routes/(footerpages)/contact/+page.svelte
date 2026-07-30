<script lang="ts">
  import Header from '$lib/components/app/Header.svelte';
  import Footer from '$lib/components/app/Footer.svelte';
  import Auth from '$lib/utils/Auth.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Alert from '$lib/components/ui/Alert.svelte';
  import { IconHexagonFilled } from '@tabler/icons-svelte-runes';
  import confetti from "@hiseb/confetti";
  import { invoke } from '@tauri-apps/api/core';

  let email = '';
  let message = '';
  let loading = false;
  let showAlert = false;
  let alertType: 'success' | 'error' = 'error';
  let alertMessage = '';
/**
 * Gets a tier of user's device performance
 *
 * This invokes a Tauri command to rust to get a calculated tier for the user's device.
 *
 * @return A promise that resolves to a string representing the device performance tier: 'low', 'medium', or 'high'.
 */
  async function getDevicePerformanceTier(): Promise<'low' | 'medium' | 'high'> {
    try {
      const tier = await invoke('get_performance_tier') as string;
      return tier as 'low' | 'medium' | 'high';
    } catch {
      return 'low';
    }
  }

  /**
   * Displays confetti animation using Confetti.js library.
   *
   * This makes use of the getDevicePerformanceTier function to determine the number of confetti
   * particles to display based on the user's device performance tier.
   *
   * This is a helper function.
   */
  async function showConfetti() {
    const root = document.documentElement;
    const primary = getComputedStyle(root).getPropertyValue('--primary-colour').trim();
    const secondary = getComputedStyle(root).getPropertyValue('--secondary-colour').trim();
    const text = getComputedStyle(root).getPropertyValue('--text-colour').trim();

    await getDevicePerformanceTier().then(tier => {
      if (tier === 'low') {
        confetti({
          count: 50,
          fade: true,
          color: [primary, secondary, text],
        });
      } else if (tier === 'medium') {
        confetti({
          count: 150,
          fade: true,
          color: [primary, secondary, text],
        });
      } else {
        confetti({
          count: 250,
          fade: true,
          color: [primary, secondary, text],
        });
      }
    });
  }

  /**
   * Handles the form submission for the contact form.
   *
   * This function validates the input fields, shows a confetti animation, and displays an alert message
   * based on the success or failure of the submission.
   */
  async function handleSubmit() {
    if (!email || !message) {
      alertMessage = 'Please fill in all fields.';
      alertType = 'error';
      showAlert = true;
      return;
    }

    loading = true;
    await showConfetti();
    alertMessage = 'Thank you for your message. We will get back to you soon.';
    alertType = 'success';
    showAlert = true;
    email = '';
    message = '';
    loading = false;
  }
</script>

<Auth redirectTo="/home" shouldBeLoggedIn={false} />
<Header />

<main class="page">
  <div class="contact-wrap">
    <div class="contact-header">
      <IconHexagonFilled size={32} color="var(--primary-colour)" />
      <h1>Contact Us</h1>
    </div>

    <Card padding="xlarge" width="60%">
      <form class="form" on:submit|preventDefault={handleSubmit}>
        <Alert
          type={alertType}
          message={alertMessage}
          show={showAlert}
        />

        <Input
          label="Email"
          type="email"
          placeholder="your@email.com"
          bind:value={email}
          required
        />

        <Input
          label="Message"
          placeholder="Tell us what's on your mind..."
          bind:value={message}
          counter={true}
          maxLength={500}
          multiline={true}
          required
        />

        <Button variant="primary" type="submit" fontSize="medium" disabled={loading}>
          {loading ? 'Sending...' : 'Send Message'}
        </Button>
      </form>
    </Card>
  </div>
</main>

<Footer />

<style>
  .contact-wrap {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 6.5rem;
  }

  .contact-header {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    text-align: center;
  }

  .contact-header h1 {
    font-size: 24px;
    font-weight: 700;
    color: var(--text-colour);
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    width: 80%;
  }
</style>