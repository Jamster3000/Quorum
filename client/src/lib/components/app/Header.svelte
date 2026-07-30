<script lang="ts">
  import { IconHexagonFilled } from '@tabler/icons-svelte-runes';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { goto } from '$app/navigation';

  import Nav from '$lib/components/ui/Nav.svelte';
  import Button from '$lib/components/ui/Button.svelte';

  const navLinks = [
    { label: 'Features', href: '/features' },
    { label: 'Self-host', href: '/selfhost' },
    { label: 'Documentation', href: 'https://jamster3000.github.io/Quorum/' },
    { label: 'GitHub', href: 'https://github.com/Jamster3000/Quorum' },
    { label: 'Contact Us', href: '/contact' },
  ];

  /**
   * Handles the nav links click events
   * 
   * For the nav links that are external URLs instead of client side pages
   * we use the Tauri openUrl function to open the link in the default browser.
   * 
   * @param link - The link object containing the label and href
   */
  async function handleNavClick(link: { label: string; href: string }) {
    if (link.label === 'GitHub' || link.label == 'Documentation') {
      await openUrl(link.href);
    } else {
      await goto(link.href);
    }
  }
</script>

<header class="header">
  <a href="/" class="header-logo">
    <IconHexagonFilled size={28} color="var(--primary-colour)" />
    <span>Quorum</span>
  </a>

  <Nav links={navLinks} onLinkClick={handleNavClick} />

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
    height: var(--header-height);
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

  .header-actions {
    display: flex;
    gap: 0.75rem;
  }
</style>
