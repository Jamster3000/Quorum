<script lang="ts">
  export let links: { label: string; href: string }[] = [];
  export let onLinkClick: ((link: { label: string; href: string }) => void) | undefined = undefined;
</script>

<nav class="nav">
  {#each links as link, i}
    <a 
      href={link.href}
      class="nav-link"
      on:click={(e) => {
        if (onLinkClick) {
          e.preventDefault();
          onLinkClick(link);
        }
      }}
    >
      {link.label}
    </a>
    {#if i < links.length - 1}
      <span class="divider">|</span>
    {/if}
  {/each}
</nav>

<style>
  .nav {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .nav-link {
    font-size: var(--font-medium);
    font-weight: 700;
    color: var(--text-colour);
    background: transparent;
    border: none;
    cursor: pointer;
    text-decoration: none;
    padding: 6px 12px;
    border-radius: 8px;
    position: relative;
    transition: color 0.15s, background 0.15s;
  }

  .divider {
    color: var(--text-colour);
    opacity: 0.3;
  }

  .nav-link::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 50%;
    width: 0;
    height: 2px;
    background: var(--primary-colour);
    border-radius: 999px;
    transform: translateX(-50%);
    transition: width 0.2s;
  }

  .nav-link:hover {
    color: var(--text-colour);
    background: color-mix(in srgb, var(--primary-colour) 10%, transparent);
  }

  .nav-link:hover::after {
    width: calc(100% - 24px);
  }
</style>