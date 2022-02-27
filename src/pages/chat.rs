use sycamore::prelude::*;
use crate::{svg::{VIDEO_SVG, IMAGE_SVG, PAPER_CLIP_SVG, SMILE_SVG, THUMBS_UP_SVG}};


#[component]
pub fn Chat<G: Html>(ctx: ScopeRef) -> View<G> {


    fn get_chat_area_style() -> String {
        let height =  web_sys::window().unwrap().inner_height().unwrap().unchecked_into_f64()-130.0;
        format!("height: {}px;", height)
    }

    fn get_chat_area_class() -> String {
        let mut chat_area_class = format!("chat-area-content");

        let inner_width =  web_sys::window().unwrap().inner_width().unwrap().unchecked_into_f64();
        if inner_width <= 574.0 {
            chat_area_class = format!("chat-area-content-inviscroll");
        }
        
        chat_area_class
    }


    view! { ctx,
        div(class="chat-area") { 
            div(id="chat-area-content", class=get_chat_area_class(), style=get_chat_area_style()) {
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
