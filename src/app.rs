use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::css::CssClass::{AboutContainer, AboutH1, AboutP, Container, HomeButton, HomeContainer, HomeH2, HomeP, MyFooter, Nav, NavLink};
use crate::css::Theme;
use crate::state::GlobalState;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let state = create_rw_signal(GlobalState::default());
    provide_context(state);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/{{project-name}}.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            // .into_view()
        }>
             <div class={move || Container.get_css(state.get().theme)}>
                <NavBar/>
                <main class="flex-grow">
                    <Routes>
                        <Route path="/" view=Home ssr=SsrMode::Async />
                        <Route path="/about" view=About ssr=SsrMode::Async />
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}


#[component]
fn NavBar() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <nav class={move || Nav.get_css(state.get().theme)}>
            <ul class="flex justify-center space-x-10 py-4">
                <li>
                    <A href="/" class={move || NavLink.get_css(state.get().theme)}>
                        {"Home"}
                    </A>
                </li>
                <li>
                    <A href="/about" class={move || NavLink.get_css(state.get().theme)}>
                        {"About"}
                    </A>
                </li>
            </ul>
        </nav>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <footer class={move || MyFooter.get_css(state.get().theme)}>
            <p class="font-bold">{"© 2024 Company Name. All rights reserved."}</p>
        </footer>
    }
}

#[component]
fn About() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <div class={move || AboutContainer.get_css(state.get().theme)}>
            <h1 class={move || AboutH1.get_css(state.get().theme)}>{"About Us"}</h1>
            <p class={move || AboutP.get_css(state.get().theme)}>{"Here you can learn more about us."}</p>
        </div>
    }
}


#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    let (count, set_count) = create_signal(1);
    view! {
        <div class={move || HomeContainer.get_css(state.get().theme)}>
            <h2 class={move || HomeH2.get_css(state.get().theme)}>"Welcome to Leptos with Tailwind"</h2>
            <p class={move || HomeP.get_css(state.get().theme)}>"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class={move || HomeButton.get_css(state.get().theme)}
                on:click=move |_| home_button_on_click(count,set_count,state)
            >
                "Something's here | "
                {move || if count.get() == 1 {
                    "Click me!".to_string()
                } else {
                    count.get().to_string()
                }}
                " | Some more text"
            </button>
        </div>
    }
}

/// 增加计数器 + 切换主题
fn home_button_on_click(count: ReadSignal<i32>, set_count: WriteSignal<i32>, state: RwSignal<GlobalState>) {
    set_count.update(|count| *count += 1);
    let current_count = count.get();
    if current_count % 2 == 0 {
        state.update(|state| state.theme = Theme::Twitter);
    } else {
        state.update(|state| state.theme = Theme::Ig);
    }
}
