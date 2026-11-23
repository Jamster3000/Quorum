<script lang="ts">
	import Header from '$components/app/Header.svelte';
	import Footer from '$components/app/Footer.svelte';
	import Auth from '$utils/Auth.svelte';
	import Card from '$components/ui/Card.svelte';
</script>

<Auth redirectTo="/home" shouldBeLoggedIn={false} />
<Header />

<main class="page">
	<Card variant="gradient">
		<h1>Unavailable</h1>
		<p>Due to the lack of server hosting - there is no server uptime status to view.</p>
	</Card>
</main>

<Footer />
