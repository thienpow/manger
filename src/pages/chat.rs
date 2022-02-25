use sycamore::prelude::*;
use crate::svg::{VIDEO_SVG, IMAGE_SVG, PAPER_CLIP_SVG, SMILE_SVG, THUMBS_UP_SVG};


#[component]
pub fn ChatArea<G: Html>(ctx: ScopeRef) -> View<G> {

    let i =  web_sys::window().unwrap().inner_height().unwrap();
    let chat_area_style = format!("height: {}px;", i.unchecked_into_f64()-130f64);

    view! { ctx,
        div(class="chat-area", style=chat_area_style) { 
            div(class="chat-area-content") {
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message1"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message2"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message3"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message4"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message1"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message2"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message3"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message4"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message"}
                div(style="padding-left: 25px; display: flex; justify-content: flex-start;"){"opponent side halo"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message1"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message2"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message3"}
                div(style="padding-right: 25px; display: flex; justify-content: flex-end;"){"my side of message4"}
            }
            div(class="chat-area-footer") {
                span(class="optinal-button", dangerously_set_inner_html=VIDEO_SVG)
                span(class="optinal-button", dangerously_set_inner_html=IMAGE_SVG)
                //span(dangerously_set_inner_html=PLUG_CIRCLE_SVG)
                span(dangerously_set_inner_html=PAPER_CLIP_SVG)
    
                input(type="text", placeholder="Type something here...")
    
                span(dangerously_set_inner_html=SMILE_SVG)
                span(class="optinal-button", dangerously_set_inner_html=THUMBS_UP_SVG)
            }
        }
        
        
    }
}
