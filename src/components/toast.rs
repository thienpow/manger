use core::fmt;
use gloo_timers::future::TimeoutFuture;
use sycamore::{prelude::*, futures::{spawn_local_scoped}};


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Positions {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight
}

impl fmt::Display for Positions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Positions::TopLeft => write!(f, "top-left"),
            Positions::TopCenter => write!(f, "top-center"),
            Positions::TopRight => write!(f, "top-right"),
            Positions::BottomLeft => write!(f, "bottom-left"),
            Positions::BottomCenter => write!(f, "bottom-center"),
            Positions::BottomRight => write!(f, "bottom-right"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToastContainerProps {
    pub position: Positions,
}

impl Default for ToastContainerProps {
    fn default() -> Self {
        Self {
            position: Positions::TopRight,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ToastProps {
    pub title: String,
    pub text: String,
    pub icon_url: String,
}


#[derive(Debug, Default, Clone)]
struct ToastState {
    pub show_new_toast: RcSignal<bool>,
    pub new_toast: RcSignal<ToastProps>,
}

pub fn show(cx: Scope, props: ToastProps) {
    let toast_state = use_context::<ToastState>(cx);
    toast_state.new_toast.set(ToastProps{title: props.title, text: props.text, icon_url: props.icon_url});
    toast_state.show_new_toast.set(true);
}

#[component]
pub fn Toast<G: Html>(cx: Scope, props: ToastContainerProps) -> View<G> {

    let show_new_toast: RcSignal<bool> = create_rc_signal(false);
    let new_toast: RcSignal<ToastProps> = create_rc_signal(ToastProps{title: "".to_string(), text: "".to_string(), icon_url: "".to_string()});
    let toast_state = ToastState {
        show_new_toast,
        new_toast
    };
    provide_context(cx, toast_state);

    let toast_container = create_node_ref(cx);

    let toast_state = use_context::<ToastState>(cx);

    create_effect(cx, move || {
        if *toast_state.show_new_toast.get() == true {
            toast_state.show_new_toast.set(false);

            let t: G = node! {cx, 
                div() {
                    ToastItem {}
                }
                
            };
            toast_container.get::<G>().append_child(&t);
        } 
    });


    view! { cx,
        div(ref=toast_container, class="toast-container", data-position=props.position.to_string())
    }
}

fn remove(cx: Scope, toast: DomNode) {
    spawn_local_scoped(cx, async move {
        TimeoutFuture::new(60).await;
        toast.add_class("hide");

        TimeoutFuture::new(230).await;
        match toast.parent_node() {
            Some(parent) => {
                parent.remove_self();
            },
            None => {}
        }
        
        toast.remove_self();
    });

}


#[component]
fn ToastItem<G: GenericNode>(cx: Scope) -> View<G> {
    let stay_show = create_signal(cx, false);
    
    let toast_state = use_context::<ToastState>(cx);
    let new_toast = toast_state.new_toast.get();
    let title = new_toast.title.to_string();
    let text = new_toast.text.to_string();
    let icon_url = new_toast.icon_url.to_string();

    let toast_ref = create_node_ref(cx);

    
    spawn_local_scoped(cx, async move {
        loop {
            TimeoutFuture::new(7000).await;
            if *stay_show.get() {
            } else {
                match toast_ref.get::<DomNode>() {
                    toast => {
                        remove(cx, toast);
                    },
                }
                break;
            }
        }
    });
    

    spawn_local_scoped(cx, async move {
        TimeoutFuture::new(60).await;
        let toast = toast_ref.get::<DomNode>();
        toast.add_class("show");
    });

    let on_click = move |_| {
        stay_show.set(false);
        remove(cx, toast_ref.get::<DomNode>());
    };

    view! { cx,
        div(ref=toast_ref, 
            class="toast",
            on:mouseenter=move |_| stay_show.set(true),
            on:mouseleave=move |_| stay_show.set(false),
        ) {
            div(class="toast-box") {
                div(class="toast-x", 
                    style=(if *stay_show.get() {"display:inline-block;"} else {""}),
                    on:click=on_click
                ) {"×"}
                div(class="toast-title") {(title)}
                div(class="toast-content") {
                    img(class="toast-img", src=(icon_url))
                    div(class="toast-text") {(text)}
                }
            }
        }
    }
}
