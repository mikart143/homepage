use super::common::GlassSection;
use leptos::prelude::*;

#[component]
pub fn ImportantLinks() -> impl IntoView {
    view! {
        <section id="links" class="w-full max-w-6xl" aria-label="Important Resources">
            <GlassSection class="p-10">
                <h2 class="text-white text-4xl font-bold mb-8 text-center focus:outline-2 focus:outline-white focus:rounded">
                    "Important Resources"
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <LinkCard
                        title="GitHub"
                        description="Check out my open-source projects and contributions"
                        url="https://github.com/mikart143"
                        icon_src="/icons/github-logo.svg"
                        icon_alt="GitHub icon"
                    />
                    <LinkCard
                        title="LinkedIn"
                        description="Connect with me professionally"
                        url="https://www.linkedin.com/in/kruczek-michal98/"
                        icon_src="/icons/linkedin-logo.svg"
                        icon_alt="LinkedIn icon"
                    />
                    <LinkCard
                        title="Blog"
                        description="Coming Soon TM"
                        url="#"
                        icon_src="/icons/blog-icon.svg"
                        icon_alt="Blog icon"
                    />
                    <LinkCard
                        title="Resume/CV"
                        description="Download my professional resume"
                        url="assets/Michal_Kruczek_Resume.pdf"
                        icon_src="/icons/document-icon.svg"
                        icon_alt="Resume icon"
                    />
                </div>
            </GlassSection>
        </section>
    }
}

#[component]
pub fn LinkCard(
    title: &'static str,
    description: &'static str,
    url: &'static str,
    icon_src: &'static str,
    icon_alt: &'static str,
) -> impl IntoView {
    let external_indicator = if url != "#" {
        " (opens in new window)"
    } else {
        ""
    };

    view! {
        <a
            href={url}
            target={if url != "#" { "_blank" } else { "_self" }}
            rel=if url != "#" { "noopener noreferrer" } else { "" }
            class="backdrop-blur-lg bg-white/5 border border-white/20 rounded-xl p-6 hover:bg-white/10 hover:scale-105 focus:outline-2 focus:outline-offset-2 focus:outline-white transition-all duration-300 group"
            aria-label={format!("{}{}", title, external_indicator)}
        >
            <div class="mb-3 h-12 w-12 rounded-lg border border-white/15 bg-white/5 flex items-center justify-center">
                <img src={icon_src} alt={icon_alt} class="h-8 w-8" loading="lazy" />
            </div>
            <h3 class="text-white text-xl font-semibold mb-2 group-hover:text-blue-400 transition-colors">
                {title}
            </h3>
            <p class="text-white/60 text-sm">
                {description}
            </p>
        </a>
    }
}
