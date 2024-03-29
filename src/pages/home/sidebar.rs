
use sycamore::{prelude::*};

use crate::svg::{BIBLE_SVG, MUSIC_SVG, GROWTH_SVG, DAILY_SVG, DONATE_SVG, PRAY_SVG, PHOTOS_SVG, FORUM_SVG};

#[component]
pub fn SidebarLeft<G: Html>(cx: Scope) -> View<G> {

    let side_bar_left_ref = create_node_ref(cx);

    /*
    
    spawn_local_scoped(cx, async move {
        TimeoutFuture::new(60).await;
        let side_bar_left = side_bar_left_ref.get::<DomNode>();
        side_bar_left.add_class("show");
    });
     */

    view! { cx,
        
        div(ref=side_bar_left_ref, class="side-bar-left") {

            /* Spiritual formation */
            div(class="side-wrapper") {

                div(class="side-title-left") {
                    "SPRITUAL FORMATION"
                }
                div(class="side-menu") {
                    a(href="#") {
                        span(dangerously_set_inner_html=BIBLE_SVG) 
                        "Discover & Learn"
                    }
                    a(href="#") {
                        span(dangerously_set_inner_html=MUSIC_SVG) 
                        "Music Moment"
                    }
                    a(href="#") {
                        span(dangerously_set_inner_html=GROWTH_SVG) 
                        "Growth Tracker"
                    }
                    a(href="#") {
                        span(dangerously_set_inner_html=DAILY_SVG) 
                        "Daily"
                    }
                }
                
            }

            /* Living in Jesus */
            div(class="side-wrapper") {

                div(class="side-title-left") {
                    "LOVE THY NEIGHBOUR"
                }
                div(class="side-menu") {
                    a(href="#") {
                        span(dangerously_set_inner_html=DONATE_SVG) 
                        "Help in Need"
                    }
                    a(href="#") {
                        span(dangerously_set_inner_html=PRAY_SVG) 
                        "Prayer Wall"
                    }
                    a(href="#") {
                        span(dangerously_set_inner_html=PHOTOS_SVG) 
                        "Snap a Joy"
                    }
                    a(href="#") {
                        span(dangerously_set_inner_html=FORUM_SVG) 
                        "Forum"
                    }
                }
                
            }


        }
        
    }
}
