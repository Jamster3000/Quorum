<script lang="ts">
	import Header from '$components/app/Header.svelte';
	import Footer from '$components/app/Footer.svelte';
	import Auth from '$utils/Auth.svelte';
</script>

<Auth redirectTo="/home" shouldBeLoggedIn={false} />
<Header />

<main class="page">
	<p> Successfull connection to selfhost</p>
</main>

<Footer />
