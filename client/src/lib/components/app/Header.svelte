<script lang="ts">
  import { IconHexagonFilled } from '@tabler/icons-svelte-runes';

  import Nav from '$lib/components/ui/Nav.svelte';
  import Button from '$lib/components/ui/Button.svelte';

  const navLinks = [
    { label: 'Features', href: '#' },
    { label: 'Self-host', href: '#' },
    { label: 'Documentation', href: '#' },
    { label: 'GitHub', href: '#' },
    { label: 'Contact Us', href: '#' },
  ];
</script>

<header class="header">
  <a href="/" class="header-logo">
    <IconHexagonFilled size={28} color="var(--primary-colour)" />
    <span>Quorum</span>
  </a>

  <Nav links={navLinks} />

  <div class="header-actions">
    <Button variant="primary" fontSize="medium" href="/signup">Sign up</Button>
    <Button variant="secondary" fontSize="medium" href="/login">Log in</Button>
  </div>
</header>

<style>
  .header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 2rem;
      height: 64px;
      background: color-mix(in srgb, var(--background-colour) 85%, var(--primary-colour));
      border-bottom: 2px solid color-mix(in srgb, var(--primary-colour) 20%, transparent);
      backdrop-filter: blur(12px);
      position: sticky;
      z-index: 10;
}

.header-logo {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: var(--font-xlarge);
  font-weight: 700;
  color: var(--text-colour);
  text-decoration: none;
  letter-spacing: 0.5px;
}

.header-logo span {
  background: linear-gradient(90deg, var(--text-colour), var(--primary-colour));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}
</style>