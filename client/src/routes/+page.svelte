<script lang="ts">
import Titlebar from '$lib/components/layout/Titlebar.svelte';
import { IconHeart } from '@tabler/icons-svelte';

</script>

<Titlebar />

<h1 style="color: var(--primary-colour);">Landing page</h1>
<p style="color: var(--secondary-colour);">This is the main landing page for users that are logged out</p>
<p style="color: var(--border-colour);">This is the main landing page for users that are logged out</p>

<br>
<p style="color: var(--text-colour);">If you would like to create an account, head to the <a href="/signup">signup page</a></p>
<br><br>

<IconHeart size={64} color="var(--primary-colour)" />


<!--Add font that's been downloaded -->