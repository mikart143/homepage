<script lang="ts">
  import GlassSection from './GlassSection.svelte';

  interface LinkItem {
    title: string;
    description: string;
    url: string;
    iconSrc: string;
    iconAlt: string;
  }

  const links: LinkItem[] = [
    {
      title: 'GitHub',
      description: 'Check out my open-source projects and contributions',
      url: 'https://github.com/mikart143',
      iconSrc: '/icons/github-logo.svg',
      iconAlt: 'GitHub icon'
    },
    {
      title: 'LinkedIn',
      description: 'Connect with me professionally',
      url: 'https://www.linkedin.com/in/kruczek-michal98/',
      iconSrc: '/icons/linkedin-logo.svg',
      iconAlt: 'LinkedIn icon'
    },
    {
      title: 'Blog',
      description: 'Coming Soon TM',
      url: '#',
      iconSrc: '/icons/blog-icon.svg',
      iconAlt: 'Blog icon'
    },
    {
      title: 'Resume/CV',
      description: 'Download my professional resume',
      url: '/assets/Michal_Kruczek_Resume.pdf',
      iconSrc: '/icons/document-icon.svg',
      iconAlt: 'Resume icon'
    }
  ];
</script>

<section id="links" class="w-full max-w-6xl" aria-label="Important Resources">
  <GlassSection class="p-10">
    <h2
      class="mb-8 text-center text-4xl font-bold text-white focus:rounded focus:outline-2 focus:outline-white"
    >
      Important Resources
    </h2>
    <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
      {#each links as { title, description, url, iconSrc, iconAlt } (title)}
        {@const external = url !== '#'}
        <a
          href={url}
          target={external ? '_blank' : '_self'}
          rel={external ? 'noopener noreferrer' : undefined}
          class="group rounded-xl border border-white/20 bg-white/5 p-6 backdrop-blur-lg transition-all duration-300 hover:scale-105 hover:bg-white/10 focus:outline-2 focus:outline-offset-2 focus:outline-white"
          aria-label={external ? `${title} (opens in new window)` : title}
        >
          <div
            class="mb-3 flex h-12 w-12 items-center justify-center rounded-lg border border-white/15 bg-white/5"
          >
            <img src={iconSrc} alt={iconAlt} class="h-8 w-8" loading="lazy" />
          </div>
          <h3
            class="mb-2 text-xl font-semibold text-white transition-colors group-hover:text-blue-400"
          >
            {title}
          </h3>
          <p class="text-sm text-white/60">{description}</p>
        </a>
      {/each}
    </div>
  </GlassSection>
</section>
