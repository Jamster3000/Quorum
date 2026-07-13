<script lang="ts">
  import { isLoggedIn } from '$lib/utils/auth';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  export let redirectTo: string = '/home';
  export let shouldBeLoggedIn: boolean = false;
  
  onMount(async () => {
    if (shouldBeLoggedIn) {
        if (!(await isLoggedIn())) {
          goto(redirectTo);
        }
    } else {
        if (await isLoggedIn()) {
            goto(redirectTo);
        }
    }
  });
  
</script>