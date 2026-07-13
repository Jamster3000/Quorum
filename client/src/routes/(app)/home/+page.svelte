<script lang="ts">
  import { goto } from '$app/navigation';
  import { clearAuthStore } from '$lib/stores/authStore';
  import Auth from '$lib/utils/Auth.svelte';

  async function logout() {
    await clearAuthStore();
    goto('/');
  }
</script>

<Auth redirectTo="/" shouldBeLoggedIn={true} />
<p>Hey, turns out you logged in successfully... Congratulations! 🎉</p>
<p>though... there's nothing here so you kinda just wasted your time coming here... sorry about that.</p>

<button on:click={logout} class="logout-button">
  Logout
</button>