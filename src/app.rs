use crate::components::*;
use leptos::prelude::*;
use leptos_meta::{Html, Meta, Script, Title, provide_meta_context};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html {..} lang="en" />
        <Title text="Michał Kruczek - Full-Stack Developer & AI Specialist" />
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Meta name="description" content="Full-stack developer specializing in .NET, AI/ML, and cloud infrastructure. Offering web development, AI solutions, and DevOps consulting." />
        <Meta name="keywords" content=".NET, C#, AI, Machine Learning, Azure, Kubernetes, Web Development, Full-Stack" />
        <Meta name="author" content="Michał Kruczek" />
        <Meta property="og:type" content="website" />
        <Meta property="og:title" content="Michał Kruczek - Full-Stack Developer & AI Specialist" />
        <Meta property="og:description" content="Expert in .NET full-stack development, AI/ML integration, and cloud infrastructure. Available for consulting and training." />
        <Meta property="og:url" content="https://kruczek.dev" />
        <Meta property="og:image" content="https://kruczek.dev/assets/profile.jpeg" />
        <Meta name="twitter:card" content="summary" />
        <Meta name="twitter:title" content="Michał Kruczek - Full-Stack Developer & AI Specialist" />
        <Meta name="twitter:description" content="Expert in .NET, AI/ML, and cloud infrastructure solutions." />
        <Meta name="twitter:image" content="https://kruczek.dev/assets/profile.jpeg" />

        //<!-- JSON-LD Structured Data -->
        <Script type_="application/ld+json">
        {r#"{
            "@context": "https://schema.org",
            "@type": "Person",
            "name": "Michał Kruczek",
            "url": "https://kruczek.dev",
            "image": "https://kruczek.dev/assets/profile.jpeg",
            "jobTitle": "Full-Stack Developer & AI Specialist",
            "sameAs": [
                "https://linkedin.com/in/kruczek-michal98",
                "https://github.com/mikart143"
            ],
            "knowsAbout": [".NET Core", "C#", "Azure", "Kubernetes", "AI/ML", "JavaScript", "React", "Angular", "Vue.js"],
            "worksFor": {
                "@type": "Organization",
                "name": "LUXMED"
            }
        }"#}
        </Script>

        <div class="relative min-h-screen bg-gray-900">
            <PlasmaBackground />

            <div class="relative z-10 flex flex-col items-center p-8 gap-16">
                // Hero Section
                <Hero />

                // Important Links Section
                <ImportantLinks />

                // Services Section
                <Services />

                // Contact Section
                <Contact />
            </div>
        </div>
    }
}
