<script lang="ts">
  import type { Snippet } from 'svelte';

  let { class: className = '', children }: { class?: string; children: Snippet } = $props();
</script>

<div
  class="rounded-2xl border border-white/20 bg-gray-900/60 shadow-xl backdrop-blur-md {className}"
>
  {@render children()}
</div>
