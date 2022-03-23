


use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

mod api;
mod pages;
mod store;
mod route;
mod svg;
mod components;
mod util;
use crate::components::toast::{Positions, ToastContainerProps, Toast};
use crate::components::{background::Background};
use crate::pages::index::Index;
use crate::store::AppState;

fn main() {
    
    sycamore::render(|cx| {
        
        //here we initialize the AppState store
        store::initialize(cx);
        
        let window = web_sys::window().unwrap();
        let app_state = use_context::<AppState>(cx);

        let inner_width_rc = app_state.inner_width.clone();
        let inner_height_rc = app_state.inner_height.clone();

        let window_resize_closure = Closure::wrap(Box::new(move || {

            let window = web_sys::window().unwrap();
            
            let inner_width: f64 = window.inner_width().unwrap().unchecked_into_f64();
            let inner_height: f64 = window.inner_height().unwrap().unchecked_into_f64();

            inner_width_rc.set(inner_width);
            inner_height_rc.set(inner_height);
            
        }) as Box<dyn FnMut()>);

        window.add_event_listener_with_callback("resize", window_resize_closure.as_ref().unchecked_ref()).unwrap();
        window_resize_closure.forget();
        
        view! { cx, 
            Background()
            Index()
            Toast(ToastContainerProps{position: Positions::TopRight})
        }
    });
}
