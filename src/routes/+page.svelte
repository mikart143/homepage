<script lang="ts">
  import Contact from '#lib/components/Contact.svelte';
  import Hero from '#lib/components/Hero.svelte';
  import ImportantLinks from '#lib/components/ImportantLinks.svelte';
  import Services from '#lib/components/Services.svelte';

  const siteUrl = 'https://kruczek.dev/';
  const title = 'Michał Kruczek - Full-Stack Developer & AI Specialist';
  const description =
    'Full-stack developer specializing in .NET, AI/ML, and cloud infrastructure. Offering web development, AI solutions, and DevOps consulting.';
  const ogDescription =
    'Expert in .NET full-stack development, AI/ML integration, and cloud infrastructure. Available for consulting and training.';
  const twitterDescription = 'Expert in .NET, AI/ML, and cloud infrastructure solutions.';
  const imageUrl = `${siteUrl}assets/profile.webp`;

  const jsonLd = {
    '@context': 'https://schema.org',
    '@type': 'Person',
    name: 'Michał Kruczek',
    url: siteUrl,
    image: imageUrl,
    jobTitle: 'Full-Stack Developer & AI Specialist',
    sameAs: ['https://linkedin.com/in/kruczek-michal98', 'https://github.com/mikart143'],
    knowsAbout: [
      '.NET Core',
      'C#',
      'Azure',
      'Kubernetes',
      'AI/ML',
      'JavaScript',
      'React',
      'Angular',
      'Vue.js'
    ],
    worksFor: {
      '@type': 'Organization',
      name: 'LUXMED'
    }
  };

  // Escape "<" so no string value can close the script tag.
  const jsonLdScript = `<script type="application/ld+json">${JSON.stringify(jsonLd).replaceAll('<', '\\u003c')}</${'script'}>`;
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />
  <meta
    name="keywords"
    content=".NET, C#, AI, Machine Learning, Azure, Kubernetes, Web Development, Full-Stack"
  />
  <meta name="author" content="Michał Kruczek" />
  <meta name="theme-color" content="#0f172a" />
  <link rel="canonical" href={siteUrl} />

  <meta property="og:type" content="website" />
  <meta property="og:title" content={title} />
  <meta property="og:description" content={ogDescription} />
  <meta property="og:url" content={siteUrl} />
  <meta property="og:image" content={imageUrl} />

  <meta name="twitter:card" content="summary" />
  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={twitterDescription} />
  <meta name="twitter:image" content={imageUrl} />

  <!-- eslint-disable-next-line svelte/no-at-html-tags -- content is JSON.stringify output with "<" escaped -->
  {@html jsonLdScript}
</svelte:head>

<Hero />
<ImportantLinks />
<Services />
<Contact />
