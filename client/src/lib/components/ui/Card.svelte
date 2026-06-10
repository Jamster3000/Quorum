<script lang="ts">
  export let padding: 'none' | 'small' | 'medium' | 'large' = 'medium';
  export let variant: 'default' | 'featured' = 'default';
</script>

<div class="card {padding} {variant}">
  <slot />
</div>

<style>
  .card {
    background: var(--card-colour);
    border: 1px solid var(--border-colour);
    border-radius: 16px;
  }

  .none { 
      padding: 0;
  }

  .small { 
      padding: 1rem;
  }

  .medium { 
      padding: 1.5rem;
  }

  .large { 
      padding: 2rem;
 }

  .featured {
    border-color: color-mix(in srgb, var(--primary-colour) 50%, transparent);
    box-shadow: 0 0 28px color-mix(in srgb, var(--primary-colour) 10%, transparent);
  }
</style>