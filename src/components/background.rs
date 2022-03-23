use sycamore::prelude::*;

use crate::store::AppState;

#[component]
pub fn Background<G: Html>(cx: Scope) -> View<G> {

    let app_state = use_context::<AppState>(cx);
    
    //let video_url = "/assets/vid/bg.mp4";
    view! { cx,
        div(class="video-bg", style=format!("background-image:url({});", &*app_state.background.get())) {
            /*
            video(autoplay=true, loop=true, muted=true) {
                source(src=video_url, type="video/mp4")
                "Your browser does not support the video tag."
            }
            */
        }

    }
}
