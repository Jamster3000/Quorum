<script lang="ts">
  import Header from '$lib/components/app/Header.svelte';
  import Footer from '$lib/components/app/footer.svelte';
  import Auth from '$lib/utils/Auth.svelte';
</script>

<Auth redirectTo="/home" shouldBeLoggedIn={false} />
<Header />

<main class="page">
	<p> Successfull connection to privacy</p>
</main>

<Footer />
