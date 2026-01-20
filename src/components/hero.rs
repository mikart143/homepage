use super::common::GlassSection;
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="flex flex-col items-center justify-center min-h-screen text-center max-w-4xl">
            <GlassSection class="p-12">
                <div class="mb-8 flex justify-center">
                    <img
                        src="assets/profile.webp"
                        alt="Michał Kruczek - Full-Stack Developer"
                        class="w-64 h-64 rounded-full border-4 border-blue-400 shadow-xl object-cover"
                    />
                </div>
                <h1 class="text-6xl md:text-7xl font-bold mb-6 bg-linear-to-r from-blue-400 via-purple-400 to-pink-400 bg-clip-text text-transparent">
                    "Welcome to My Portfolio"
                </h1>
                <p class="text-white/90 text-xl md:text-2xl mb-8 leading-relaxed">
                    "Full-Stack Developer & Creative Problem Solver"
                </p>
                <p class="text-white/70 text-lg mb-10 max-w-2xl mx-auto">
                    "I build modern, scalable web applications with cutting-edge technologies.
                    Passionate about creating elegant solutions to complex problems."
                </p>
                <div class="flex gap-4 justify-center flex-wrap">
                    <a href="#services" class="px-8 py-3 bg-linear-to-r from-blue-500 to-purple-600 text-white rounded-lg font-semibold hover:scale-105 transition-transform shadow-lg">
                        "View Services"
                    </a>
                    <a href="#contact" class="px-8 py-3 backdrop-blur-lg bg-white/10 border border-white/30 text-white rounded-lg font-semibold hover:scale-105 transition-transform">
                        "Get in Touch"
                    </a>
                </div>
            </GlassSection>
        </section>
    }
}
