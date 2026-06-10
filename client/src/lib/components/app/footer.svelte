<script lang="ts">
  import { IconHexagonFilled } from '@tabler/icons-svelte';

  import Nav from '$lib/components/ui/Nav.svelte';

  const footerLinks = [
    { label: 'Privacy', href: '#' },
    { label: 'Terms', href: '#' },
    { label: 'Docs', href: '#' },
    { label: 'GitHub', href: '#' },
  ];
</script>

<footer class="footer">
  <div class="footer-logo">
    <IconHexagonFilled size={18} color="var(--primary-colour)" />
    <span>Quorum</span>
  </div>

  <Nav links={footerLinks} />

  <p class="footer-copy">© 2026 Quorum</p>
</footer>

<style>
  .footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 2rem;
    height: 52px;
    background: color-mix(in srgb, var(--background-colour) 85%, var(--primary-colour));
    border-top: 1px solid color-mix(in srgb, var(--primary-colour) 30%, transparent);
  }

  .footer-logo {
    display: flex;
    align-items: center;
    gap: 7px;
    font-size: var(--font-medium);
    font-weight: 700;
    color: var(--text-colour);
  }

  .footer-logo span {
    background: linear-gradient(90deg, var(--text-colour), var(--primary-colour));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .footer-copy {
    font-size: var(--font-small);
    font-weight: 600;
    color: var(--text-muted-colour);
  }
</style>