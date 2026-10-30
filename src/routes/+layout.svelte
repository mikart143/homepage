<script lang="ts">
  import '../app.css';
  import PlasmaBackground from '#lib/components/PlasmaBackground.svelte';

  let { children } = $props();
</script>

<PlasmaBackground />

<a
  href="#main"
  class="sr-only focus:not-sr-only focus:absolute focus:top-0 focus:left-0 focus:bg-blue-500 focus:p-4 focus:text-white"
>
  Skip to main content
</a>

<main id="main" class="relative z-10 flex flex-col items-center gap-16 p-8">
  {@render children()}
</main>
