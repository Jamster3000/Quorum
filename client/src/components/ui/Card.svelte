<script lang="ts">
  export let padding: 'none' | 'small' | 'medium' | 'large' | 'xlarge' = 'medium';
  export let width: string = '100%'; //Can be set to auto for it to work its own width out based on content.
  export let height: string = 'auto'; //Can be set to auto for it to work its own height out based on content.
  export let center: boolean = true;
  export let variant: 'default' | 'gradient' = 'default';
</script>

<div class="card {padding} {variant}" class:center style="--card-width: {width}; --card-height: {height};">
  <slot />
</div>

<style>
  :root {
    --card-width: 100%;
    --card-height: auto;
  }

  .card {
    width: var(--card-width);
    height: var(--card-height);
    border-radius: 12px;
    border: 1px solid var(--border-colour);
  }

  .card.center {
    margin-left: auto;
    margin-right: auto;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .default {
    background: var(--card-colour);
  }

  .gradient {
    background: linear-gradient(135deg, var(--card-colour) 0%, color-mix(in srgb, var(--primary-colour) 25%, var(--card-colour)) 43%, var(--card-colour) 100%);
    border: 1px solid;
    border-image: linear-gradient(135deg, transparent, color-mix(in srgb, var(--secondary-colour) 40%, transparent), transparent) 1;
    box-shadow: 0 0.25rem 0.5rem -0.25rem rgba(0, 0, 0, 0.2);
  }

  .none { 
    padding: 0;
  }

  .small { 
    padding: 1.5rem;
  }

  .medium { 
    padding: 2rem;
  }

  .large { 
    padding: 2.5rem;
  }

  .xlarge {
    padding: 4rem;
  }
</style>