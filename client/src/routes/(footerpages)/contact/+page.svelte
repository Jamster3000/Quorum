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

  let email = '';
  let message = '';
  let loading = false;
  let showAlert = false;
  let alertType: 'success' | 'error' = 'error';
  let alertMessage = '';

  function showConfetti() {
    const root = document.documentElement;
    const primary = getComputedStyle(root).getPropertyValue('--primary-colour').trim();
    const secondary = getComputedStyle(root).getPropertyValue('--secondary-colour').trim();
    const text = getComputedStyle(root).getPropertyValue('--text-colour').trim();

    confetti({
      count: 200,
      fade: true,
      color: [primary, secondary, text],
    });
  }

  async function handleSubmit() {
    if (!email || !message) {
      alertMessage = 'Please fill in all fields.';
      alertType = 'error';
      showAlert = true;
      return;
    }

    loading = true;
    showConfetti();
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