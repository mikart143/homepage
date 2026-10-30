<script lang="ts">
  import GlassSection from './GlassSection.svelte';

  interface Service {
    title: string;
    description: string;
    features: string[];
  }

  const services: Service[] = [
    {
      title: '.NET Full-Stack Development',
      description:
        'Enterprise applications using .NET Core, ASP.NET Core, and modern frontend frameworks',
      features: ['ASP.NET Core', 'Angular/React/Vue.js', 'MSSQL Database']
    },
    {
      title: 'AI & Machine Learning',
      description:
        'AI software development, infrastructure setup, and AI-based system implementations',
      features: ['OpenAI Integration', 'PyTorch & Pandas', 'FastAPI Backend']
    },
    {
      title: 'Cloud Infrastructure & DevOps',
      description:
        'Scalable cloud solutions with Azure, Kubernetes, and microservices architecture',
      features: ['Azure Cloud', 'Kubernetes', 'Microservices & CI/CD']
    },
    {
      title: 'DevOps & Deployment',
      description: 'Automated CI/CD pipelines and production deployment strategies',
      features: ['Docker', 'Kubernetes', 'Azure DevOps & Git']
    },
    {
      title: 'Consulting',
      description: 'Technical guidance, architecture planning, and team leadership',
      features: ['Code Review', 'Tech Stack Selection', 'Best Practices']
    },
    {
      title: 'AI Training & Workshops',
      description:
        'Training sessions on GitHub Copilot, LLM integration, and AI tools for enterprise teams',
      features: ['GitHub Copilot', 'LLM Best Practices', 'Team Upskilling']
    }
  ];
</script>

<section id="services" class="w-full max-w-6xl" aria-label="Services">
  <GlassSection class="p-10">
    <h2
      class="mb-4 text-center text-4xl font-bold text-white focus:rounded focus:outline-2 focus:outline-white"
    >
      Services I Provide
    </h2>
    <p class="mx-auto mb-10 max-w-2xl text-center text-white/70">
      Comprehensive solutions tailored to your needs
    </p>
    <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
      {#each services as { title, description, features } (title)}
        <article
          class="rounded-xl border border-white/20 bg-white/5 p-6 backdrop-blur-lg transition-all duration-300 focus-within:rounded focus-within:outline-2 focus-within:outline-white hover:bg-white/10"
        >
          <h3
            class="mb-3 text-2xl font-semibold text-white focus:rounded focus:outline-2 focus:outline-white"
          >
            {title}
          </h3>
          <p class="mb-4 text-sm leading-relaxed text-white/70">{description}</p>
          <ul class="space-y-2" aria-label="Features of {title}">
            {#each features as feature (feature)}
              <li
                class="flex items-center gap-2 text-sm text-white/60 focus:rounded focus:outline-2 focus:outline-white"
              >
                <span class="text-green-400" aria-hidden="true">✓</span>
                {feature}
              </li>
            {/each}
          </ul>
        </article>
      {/each}
    </div>
  </GlassSection>
</section>
