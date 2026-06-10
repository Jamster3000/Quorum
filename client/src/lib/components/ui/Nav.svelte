<script lang="ts">
  export let links: { label: string; href: string }[] = [];
</script>

<nav class="nav">
  {#each links as link}
    <a href={link.href}>{link.label}</a>
  {/each}
</nav>

<style>
  .nav {
    display: flex;
    align-items: center;
    gap: 2rem;
  }

  .nav a {
    font-size: var(--font-medium);
    font-weight: 600;
    color: var(--text-muted-colour);
    text-decoration: none;
    padding: 4px 0;
    position: relative;
    transition: color 0.15s;
  }

  .nav a::after {
    content: '';
    position: absolute;
    bottom: -2px;
    left: 0;
    width: 0;
    height: 2px;
    background: var(--primary-colour);
    border-radius: 999px;
    transition: width 0.2s;
  }

  .nav a:hover {
    color: var(--text-colour);
  }

  .nav a:hover::after {
    width: 100%;
  }
</style>