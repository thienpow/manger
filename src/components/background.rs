use sycamore::prelude::*;

use crate::store::AppState;

#[component]
pub fn Background<G: Html>(ctx: ScopeRef) -> View<G> {

    let app_state = ctx.use_context::<AppState>();
    
    //let video_url = "/assets/vid/bg.mp4";
    view! { ctx,
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
