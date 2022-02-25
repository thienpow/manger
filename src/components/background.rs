use sycamore::prelude::*;


#[component]
pub fn Background<G: Html>(ctx: ScopeRef) -> View<G> {
    
    //let video_url = "/assets/vid/bg.mp4";
    let image_url = "/assets/img/bg.webp";
    let BG_STR= format!("background-image:url({});", image_url);

    view! { ctx,
        div(class="video-bg", style=BG_STR) {
            /* 
            video(autoplay=true, loop=true, muted=true) {
                source(src=video_url, type="video/mp4")
                "Your browser does not support the video tag."
            }
            */
        }
        
    }
}

pub fn _set_background(url: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.set_attribute("background-image", url).unwrap();
}