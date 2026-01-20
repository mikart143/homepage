use super::common::GlassSection;
use leptos::prelude::*;

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <section id="services" class="w-full max-w-6xl" aria-label="Services">
            <GlassSection class="p-10">
                <h2 class="text-white text-4xl font-bold mb-4 text-center focus:outline-2 focus:outline-white focus:rounded">
                    "Services I Provide"
                </h2>
                <p class="text-white/70 text-center mb-10 max-w-2xl mx-auto">
                    "Comprehensive solutions tailored to your needs"
                </p>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <ServiceCard
                        title=".NET Full-Stack Development"
                        description="Enterprise applications using .NET Core, ASP.NET Core, and modern frontend frameworks"
                        features=vec!["ASP.NET Core", "Angular/React/Vue.js", "MSSQL Database"]
                    />
                    <ServiceCard
                        title="AI & Machine Learning"
                        description="AI software development, infrastructure setup, and AI-based system implementations"
                        features=vec!["OpenAI Integration", "PyTorch & Pandas", "FastAPI Backend"]
                    />
                    <ServiceCard
                        title="Cloud Infrastructure & DevOps"
                        description="Scalable cloud solutions with Azure, Kubernetes, and microservices architecture"
                        features=vec!["Azure Cloud", "Kubernetes", "Microservices & CI/CD"]
                    />
                    <ServiceCard
                        title="DevOps & Deployment"
                        description="Automated CI/CD pipelines and production deployment strategies"
                        features=vec!["Docker", "Kubernetes", "Azure DevOps & Git"]
                    />
                    <ServiceCard
                        title="Consulting"
                        description="Technical guidance, architecture planning, and team leadership"
                        features=vec!["Code Review", "Tech Stack Selection", "Best Practices"]
                    />
                    <ServiceCard
                        title="AI Training & Workshops"
                        description="Training sessions on GitHub Copilot, LLM integration, and AI tools for enterprise teams"
                        features=vec!["GitHub Copilot", "LLM Best Practices", "Team Upskilling"]
                    />
                </div>
            </GlassSection>
        </section>
    }
}

#[component]
pub fn ServiceCard(
    title: &'static str,
    description: &'static str,
    features: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <article class="backdrop-blur-lg bg-white/5 border border-white/20 rounded-xl p-6 hover:bg-white/10 focus-within:outline-2 focus-within:outline-white focus-within:rounded transition-all duration-300">
            <h3 class="text-white text-2xl font-semibold mb-3 focus:outline-2 focus:outline-white focus:rounded">
                {title}
            </h3>
            <p class="text-white/70 mb-4 text-sm leading-relaxed">
                {description}
            </p>
            <ul class="space-y-2" aria-label={format!("Features of {}", title)}>
                {features
                    .into_iter()
                    .map(|feature| {
                        view! {
                            <li class="text-white/60 text-sm flex items-center gap-2 focus:outline-2 focus:outline-white focus:rounded">
                                <span class="text-green-400" aria-hidden="true">"✓"</span>
                                {feature}
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </article>
    }
}
