<script>
  import '../app.css';
  import { fade } from 'svelte/transition';
  import { navigating } from '$app/stores';
</script>

{#key $navigating?.to?.url.pathname}
  <div in:fade={{ duration: 450 }}>
    <slot />
  </div>
{/key}