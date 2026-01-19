use leptos::prelude::*;

#[component]
pub fn GlassSection(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class={format!(
            "backdrop-blur-md bg-white/10 rounded-2xl border border-white/20 shadow-xl {}",
            class
        )}>
            {children()}
        </div>
    }
}
