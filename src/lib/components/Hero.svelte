<script lang="ts">
  import GlassSection from './GlassSection.svelte';
</script>

<section
  class="flex min-h-screen max-w-4xl flex-col items-center justify-center text-center"
  aria-label="Hero"
>
  <GlassSection class="p-12">
    <div class="mb-8 flex justify-center">
      <img
        src="/assets/profile.webp"
        alt="Michał Kruczek - Full-Stack Developer"
        class="h-64 w-64 rounded-full border-4 border-blue-400 object-cover shadow-xl"
        fetchpriority="high"
      />
    </div>
    <h1
      class="mb-6 bg-linear-to-r from-blue-400 via-purple-400 to-pink-400 bg-clip-text text-6xl font-bold text-transparent focus:rounded focus:outline-2 focus:outline-white md:text-7xl"
    >
      Welcome to My Portfolio
    </h1>
    <p class="mb-8 text-xl leading-relaxed text-white/90 md:text-2xl">
      Full-Stack Developer &amp; Creative Problem Solver
    </p>
    <p class="mx-auto mb-10 max-w-2xl text-lg text-white/70">
      I build modern, scalable web applications with cutting-edge technologies. Passionate about
      creating elegant solutions to complex problems.
    </p>
    <div class="flex flex-wrap justify-center gap-4">
      <a
        href="#services"
        class="rounded-lg bg-linear-to-r from-blue-500 to-purple-600 px-8 py-3 font-semibold text-white shadow-lg transition-transform hover:scale-105 focus:outline-2 focus:outline-offset-2 focus:outline-white"
        aria-label="Jump to services section"
      >
        View Services
      </a>
      <a
        href="#contact"
        class="rounded-lg border border-white/30 bg-white/10 px-8 py-3 font-semibold text-white backdrop-blur-lg transition-transform hover:scale-105 focus:outline-2 focus:outline-offset-2 focus:outline-white"
        aria-label="Jump to contact section"
      >
        Get in Touch
      </a>
    </div>
  </GlassSection>
</section>
