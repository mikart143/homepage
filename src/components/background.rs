use leptos::prelude::*;

#[component]
pub fn PlasmaBackground() -> impl IntoView {
    view! {
        <div class="absolute top-0 left-0 w-full h-full overflow-hidden z-0 blur-[150px]">
            <div
                class="absolute rounded-full opacity-60 mix-blend-screen animate-gradient-1 -z-2"
                style="background: rgb(255, 0, 0); width: 700px; height: 700px; left: 60%; top: 40%;"
            />
            <div
                class="absolute rounded-full opacity-60 mix-blend-screen animate-gradient-2 -z-1"
                style="background: rgb(0, 255, 0); width: 600px; height: 600px; left: 40%; top: 60%;"
            />
            <div
                class="absolute rounded-full opacity-60 mix-blend-screen animate-gradient-3 -z-3"
                style="background: rgb(0, 0, 255); width: 500px; height: 500px; left: 50%; top: 50%;"
            />
        </div>
    }
}
