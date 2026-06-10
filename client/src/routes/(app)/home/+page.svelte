<script lang="ts">
  import { goto } from '$app/navigation';
  import { Store } from '@tauri-apps/plugin-store';
  import Auth from '$lib/utils/Auth.svelte';

  async function logout() {
    const store = await Store.load('.auth.dat');
    await store.clear();
    await store.save();
    goto('/');
  }
</script>

<Auth redirectTo="/" shouldBeLoggedIn={true} />
<p>Hey, turns out you logged in successfully... Congratulations! 🎉</p>
<p>though... there's nothing here so you kinda just wasted your time coming here... sorry about that.</p>

<button on:click={logout} class="logout-button">
  Logout
</button>