

use sycamore::prelude::*;

use crate::store::AppState;

#[component]
pub fn Content<G: Html>(ctx: Scope) -> View<G> {
    let app_state = ctx.use_context::<AppState>();
    
    let chat_content_style = ctx.create_memo(|| {
        let inner_width: f64 = *app_state.inner_width.get();
        let inner_height: f64 = *app_state.inner_height.get();
        let mut height =  inner_height-188.0;
        if inner_width <= 738.0 {
            height =  inner_height-130.0;
        } 
        format!("height: {}px;", height)
    });

    view! { ctx,
        article(
            id="chat-content", 
            class="chat-content", 
            style=*chat_content_style.get()
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
    }
}