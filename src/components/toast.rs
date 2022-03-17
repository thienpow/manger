use core::fmt;
use gloo_timers::future::TimeoutFuture;
use sycamore::{prelude::*, futures::ScopeSpawnLocal};

//#[allow(dead_code)]
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
pub struct ToastState {
    pub show_new_toast: RcSignal<bool>,
    pub new_toast: RcSignal<ToastProps>,
}

pub fn Show(ctx: Scope, props: ToastProps) {
    let toast_state = ctx.use_context::<ToastState>();
    toast_state.new_toast.set(ToastProps{title: props.title, text: props.text, icon_url: props.icon_url});
    toast_state.show_new_toast.set(true);
}

#[component]
pub fn Toast<G: Html>(ctx: Scope, props: ToastContainerProps) -> View<G> {

    let show_new_toast: RcSignal<bool> = create_rc_signal(false);
    let new_toast: RcSignal<ToastProps> = create_rc_signal(ToastProps{title: "".to_string(), text: "".to_string(), icon_url: "".to_string()});
    let toast_state = ToastState {
        show_new_toast,
        new_toast
    };
    ctx.provide_context(toast_state);

    let toast_container = ctx.create_node_ref();

    let toast_state = ctx.use_context::<ToastState>();

    ctx.create_effect(move || {
        if *toast_state.show_new_toast.get() == true {
            toast_state.show_new_toast.set(false);

            let t: G = node! {ctx, 
                div() {
                    ToastItem {}
                }
                
            };
            toast_container.get::<G>().append_child(&t);
        } 
    });


    view! { ctx,
        div(ref=toast_container, class="toast-container", data-position=props.position.to_string())
    }
}


#[component]
fn ToastItem<G: GenericNode>(ctx: Scope) -> View<G> {
    let toast_state = ctx.use_context::<ToastState>();
    let new_toast = toast_state.new_toast.get();
    let title = new_toast.title.to_string();
    let text = new_toast.text.to_string();

    let toast = ctx.create_node_ref();

    let remove = move || {
        let toast = toast.get::<DomNode>();
        let parent = toast.parent_node().unwrap();
        
        parent.remove_self();
        toast.remove_self();
    };

    let on_click = move |_| {
        remove();
    };

    ctx.spawn_local(async move {
        TimeoutFuture::new(3000).await;
        remove();
    });

    ctx.spawn_local(async move {
        TimeoutFuture::new(60).await;
        let toast = toast.get::<DomNode>();
        toast.add_class("show");
    });

    view! { ctx,
        div(
            ref=toast, 
            class="toast",
            on:click=on_click
    ) {
            //img(class="toast-img", src=(icon_url))
            div(class="toast-title") {(title)}
            div(class="toast-text") {(text)}
            
        }
    }
}
