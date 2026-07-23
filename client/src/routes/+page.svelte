<script lang="ts">
  import Header from '$lib/components/app/Header.svelte';
  import Footer from '$lib/components/app/footer.svelte';
  import Auth from '$lib/utils/Auth.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import Card from '$lib/components/ui/Card.svelte';
</script>

<Auth redirectTo="/home" shouldBeLoggedIn={false} />
<Header />

<main class="page">
	<p>This will be where the unauthorised landing page content will go</p>
</main>

<Footer />
