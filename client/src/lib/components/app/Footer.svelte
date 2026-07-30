<script lang="ts">
  import { IconHexagonFilled } from '@tabler/icons-svelte-runes';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { goto } from '$app/navigation';

  import Nav from '$lib/components/ui/Nav.svelte';

  const footerLinks = [
    { label: 'Privacy', href:'/privacy' },
    { label: 'Terms', href:'/terms' },
    { label: 'Contact Us', href:'/contact' },
    { label: 'Documentation', href: 'https://jamster3000.github.io/Quorum/' },
    { label: 'GitHub', href:'https://github.com/Jamster3000/Quorum' },
    { label: 'Status Page', href:'/status' },
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
    if (link.label === 'GitHub' || link.label === 'Documentation') {
      await openUrl(link.href);
    } else {
      await goto(link.href);
    }
  }
</script>

<footer class="footer">
  <div class="footer-logo">
    <IconHexagonFilled size={18} color="var(--primary-colour)" />
    <span>Quorum</span>
  </div>

  <Nav links={footerLinks} onLinkClick={handleNavClick} />

  <p class="footer-copy">© 2026 Quorum</p>
</footer>

<style>
  .footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 2rem;
    height: var(--footer-height);
    background: color-mix(in srgb, var(--background-colour) 85%, var(--primary-colour));
    border-top: 2px solid color-mix(in srgb, var(--primary-colour) 20%, transparent);
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
    color: var(--text-colour);
  }
</style>
