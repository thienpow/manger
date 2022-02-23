use sycamore::prelude::*;
use crate::svg::{LIGHT_BULB_SVG, CLOUD_MOON_SVG};
use crate::context::DarkMode;

#[component]
pub fn ToggleMode<G: Html>(ctx: ScopeRef) -> View<G> {

    let DarkMode(dark_mode) = ctx.use_context::<DarkMode>();
    let toggle = |_| {
        dark_mode.set(!*dark_mode.get());

        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().class_list().toggle("light-mode").expect("");

    };

    view! { ctx,
        button(
            class=if *dark_mode.get() { "dark-light" } else { "dark-light light-mode" },
            aria-label="Night Mode button",
            on:click=toggle,
            dangerously_set_inner_html=if *dark_mode.get() { CLOUD_MOON_SVG } else { LIGHT_BULB_SVG },
        )
    }
}

