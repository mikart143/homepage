use super::common::GlassSection;
use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section id="contact" class="w-full max-w-4xl mb-20" aria-label="Contact">
            <GlassSection class="p-10">
                <h2 class="text-white text-4xl font-bold mb-4 text-center focus:outline-2  focus:rounded">
                    "Get in Touch"
                </h2>
                <p class="text-white/70 text-center mb-10">
                    "Let's discuss your next project or collaboration opportunity"
                </p>

                <div class="backdrop-blur-lg bg-white/5 border border-white/20 rounded-xl p-8">
                    <form action="https://submit-form.com/A9BYhfqrW" method="POST" class="space-y-6" aria-label="Contact form">
                        <div class="form-group">
                            <label for="name" class="block text-white/70 text-sm font-medium mb-2">
                                "Name"
                                <span class="text-red-400" aria-label="required">"*"</span>
                            </label>
                            <input
                                type="text"
                                id="name"
                                name="name"
                                placeholder="Your name"
                                required=true
                                aria-required="true"
                                class="w-full px-4 py-3 bg-white/5 border border-white/20 rounded-lg text-white placeholder-white/40 focus:outline-2 focus:outline-offset-2 focus:outline-blue-400 transition-colors"
                            />
                        </div>
                        <div class="form-group">
                            <label for="email" class="block text-white/70 text-sm font-medium mb-2">
                                "Email"
                                <span class="text-red-400" aria-label="required">"*"</span>
                            </label>
                            <input
                                type="email"
                                id="email"
                                name="email"
                                placeholder="your.email@example.com"
                                required=true
                                aria-required="true"
                                class="w-full px-4 py-3 bg-white/5 border border-white/20 rounded-lg text-white placeholder-white/40 focus:outline-2 focus:outline-offset-2 focus:outline-blue-400 transition-colors"
                            />
                        </div>
                        <div class="form-group">
                            <label for="message" class="block text-white/70 text-sm font-medium mb-2">
                                "Message"
                                <span class="text-red-400" aria-label="required">"*"</span>
                            </label>
                            <textarea
                                id="message"
                                name="message"
                                placeholder="Tell me about your project or inquiry..."
                                rows="5"
                                required=true
                                aria-required="true"
                                class="w-full px-4 py-3 bg-white/5 border border-white/20 rounded-lg text-white placeholder-white/40 focus:outline-2 focus:outline-offset-2 focus:outline-blue-400 transition-colors resize-none"
                            />
                        </div>
                        <button
                            type="submit"
                            class="w-full py-3 bg-linear-to-r from-blue-500 to-purple-600 text-white rounded-lg font-semibold hover:scale-[1.02] focus:outline-2 focus:outline-offset-2 focus:outline-blue-400 transition-transform shadow-lg"
                            aria-label="Send your message"
                        >
                            "Send Message"
                        </button>
                    </form>
                </div>
            </GlassSection>
        </section>
    }
}

#[component]
pub fn ContactInfo(
    icon: &'static str,
    label: &'static str,
    value: &'static str,
    link: &'static str,
) -> impl IntoView {
    view! {
        {move || {
            if link.is_empty() {
                view! {
                    <div class="backdrop-blur-lg bg-white/5 border border-white/20 rounded-xl p-6">
                        <div class="flex items-start gap-4">
                            <div class="text-3xl">{icon}</div>
                            <div>
                                <div class="text-white/60 text-sm mb-1">{label}</div>
                                <div class="text-white font-medium">{value}</div>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <a
                        href={link}
                        class="backdrop-blur-lg bg-white/5 border border-white/20 rounded-xl p-6 hover:bg-white/10 transition-all duration-300 block"
                    >
                        <div class="flex items-start gap-4">
                            <div class="text-3xl">{icon}</div>
                            <div>
                                <div class="text-white/60 text-sm mb-1">{label}</div>
                                <div class="text-white font-medium">{value}</div>
                            </div>
                        </div>
                    </a>
                }.into_any()
            }
        }}
    }
}
