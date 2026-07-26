<script lang="ts">
  import Header from '$lib/components/app/Header.svelte';
  import Footer from '$lib/components/app/Footer.svelte';
  import Auth from '$lib/utils/Auth.svelte';
</script>

<Auth redirectTo="/home" shouldBeLoggedIn={false} />
<Header />

<main class="page">
	<p> Successfull connection to status</p>
</main>

<Footer />
