<script lang="ts">
	import Header from '$components/app/Header.svelte';
	import Footer from '$components/app/Footer.svelte';
	import Auth from '$lib/utils/Auth.svelte';
</script>

<Auth redirectTo="/home" shouldBeLoggedIn={false} />
<Header />

<main class="page">
	<p>This will be where the unauthorised landing page content will go</p>
</main>

<Footer />
