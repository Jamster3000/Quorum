<script lang="ts">
  import Titlebar from '$lib/components/layout/Titlebar.svelte';
  import Header from '$lib/components/app/Header.svelte';
  import Footer from '$lib/components/app/Footer.svelte';
  import Auth from '$lib/utils/Auth.svelte';
</script>

<Auth redirectTo="/home" shouldBeLoggedIn={false} />
<Titlebar />
<Header />

<main class="page">
	<p>This will be where the unauthorised landing page content will go</p>
</main>

<Footer />