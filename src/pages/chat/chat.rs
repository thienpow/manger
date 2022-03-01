use sycamore::prelude::*;
use crate::{svg::{IMAGE_SVG, PAPER_CLIP_SVG, SMILE_SVG, THUMBS_UP_SVG}, components::contactbar::ContactBar};
use crate::pages::chat::navbar::NavBar;

#[component]
pub fn Chat<G: Html>(ctx: ScopeRef) -> View<G> {


    fn get_chat_area_style() -> String {
        let mut height =  web_sys::window().unwrap().inner_height().unwrap().unchecked_into_f64()-188.0;
        let width: f64 = web_sys::window().unwrap().inner_width().unwrap().unchecked_into_f64();

        if width <= 540.0 {
            height =  web_sys::window().unwrap().inner_height().unwrap().unchecked_into_f64()-130.0;
        } 
        format!("height: {}px;", height)
        
    }

    view! { ctx,

        div(class="wrapper") {

            div(class="main-container") {

                div(class="chat-wrapper") { 
                    NavBar()
                    div(
                        id="chat-content", 
                        class="chat-content", 
                        style=get_chat_area_style()
                    ) {
                        div(){"1 opponent side halo"}
                        div(){"my side of message"}
                        div(){"opponent side halo"}
                        div(){"my side of message"}
                        div(){"opponent side halo"}
                        div(){"my side of message"}
                        div(){"opponent side halo"}
                        div(){"my side of message1"}
                        div(){"my side of message2"}
                        div(){"my side of message3"}
                        div(){"my side of message4"}
                        div(){"opponent side halo"}
                        div(){"my side of message"}
                        div(){"opponent side halo"}
                        div(){"my side of message"}
                        div(){"opponent side halo"}
                        div(){"my side of message"}
                        div(){"opponent side halo"}
                        div(){"my side of message1"}
                        div(){"my side of message2"}
                        div(){"my side of message3"}
                        div(){"my side of message4"}
                        div(){"opponent side halo"}
                        div(){"my side of message"}
                        div(){"opponent side halo"}
                        div(){"my side of message"}
                        div(){"opponent side halo"}
                        div(){"my side of message"}
                        div(){"opponent side halo"}
                        div(){"my side of message1"}
                        div(){"my side of message2"}
                        div(){"my side of message3"}
                        div(){"99my side of message4"}
                    }
                    div(class="chat-footer") {
                        span(class="optinal-button", dangerously_set_inner_html=IMAGE_SVG)
                        //span(dangerously_set_inner_html=PLUG_CIRCLE_SVG)
                        span(dangerously_set_inner_html=PAPER_CLIP_SVG)
            
                        input(type="text", placeholder="Type something here...")
            
                        span(dangerously_set_inner_html=SMILE_SVG)
                        span(class="optinal-button", dangerously_set_inner_html=THUMBS_UP_SVG)
                    }
                }
                
            }

            ContactBar()
        }

        
    }
}
