
use sycamore::prelude::*;
use sycamore_router::{Route};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::console;

mod pages;
mod context;
mod svg;
mod components;
mod global;
use crate::components::{background::Background};
use crate::context::{DarkMode, CurrentRoute, LeftMenuOpened, BackgroundImage, BackgroundVideo};
use crate::pages::app::App;

#[derive(Debug, Route)]
pub  enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/bible")]
    BibleStudy,
    #[to("/love")]
    Love,
    #[to("/community")]
    Community,
    #[to("/chat/<id>")]
    Chat(String),
    #[to("/profile")]
    Profile,
    #[not_found]
    NotFound,
}

fn main() {
    /*
        function resizeListener() {
            heightOutput.textContent = window.innerHeight;
            widthOutput.textContent = window.innerWidth;
        }

        window.addEventListener("resize", resizeListener);
    */

    sycamore::render(|ctx| {

        
        let local_storage = web_sys::window().unwrap().local_storage().unwrap();
        
        // Get dark mode from media query.
        let dark_mode_mq = web_sys::window()
            .unwrap()
            .match_media("(prefers-color-scheme: light)")
            .unwrap()
            .unwrap()
            .matches();
            
        let dark_mode = if let Some(local_storage) = &local_storage {
            let dark_mode_ls = local_storage.get_item("dark_mode").unwrap();
            dark_mode_ls.as_deref() == Some("true") || (dark_mode_ls.is_none() && dark_mode_mq)
        } else {
            dark_mode_mq
        };
        
        if !dark_mode {
            let document = web_sys::window().unwrap().document().unwrap();
            document.body().unwrap().class_list().toggle("light-mode").expect("");
        }

        let dark_mode = DarkMode(create_rc_signal(dark_mode));
        ctx.provide_context(dark_mode);

        let DarkMode(dark_mode) = ctx.use_context::<DarkMode>();

        ctx.create_effect(move || {
            if let Some(local_storage) = &local_storage {
                local_storage
                    .set_item("dark_mode", &*dark_mode.get().to_string())
                    .unwrap();
            }
        });

        let current_route = CurrentRoute(create_rc_signal(AppRoutes::Home));
        ctx.provide_context(current_route);

        let left_menu_opened = LeftMenuOpened(create_rc_signal(false));
        ctx.provide_context(left_menu_opened);

        let local_storage = web_sys::window().unwrap().local_storage().unwrap();
        let _background_image = if let Some(local_storage) = &local_storage {
            let background_image_ls = local_storage.get_item("background_image").unwrap();
            if background_image_ls.is_none() {
                "".to_string()
            } else {
                background_image_ls.unwrap()
            }
        } else { 
            "".to_string()
        };

        let background_image = BackgroundImage(create_rc_signal("".to_string()));
        ctx.provide_context(background_image);

        let _background_video = if let Some(local_storage) = &local_storage {
            let background_video_ls = local_storage.get_item("background_video").unwrap();
            if background_video_ls.is_none() {
                "".to_string()
            } else {
                background_video_ls.unwrap()
            }
        } else { 
            "".to_string()
        };

        let background_video = BackgroundVideo(create_rc_signal("".to_string()));
        ctx.provide_context(background_video);

            
        let window_resize_closure = Closure::wrap(Box::new(move |_: web_sys::UiEvent| {

            let mut height: f64 =  web_sys::window().unwrap().inner_height().unwrap().unchecked_into_f64()-188.0;
            let width: f64 = web_sys::window().unwrap().inner_width().unwrap().unchecked_into_f64();

            console::log_1(&format!("window_resize_closure").as_str().into());

            let chat_content = web_sys::window().unwrap().document().unwrap().get_element_by_id("chat-content");
            if chat_content.is_some() {
                let chat_content = chat_content.unwrap();
                
                if width <= 540.0 {
                    height =  web_sys::window().unwrap().inner_height().unwrap().unchecked_into_f64()-130.0;
                    
                }
                chat_content.set_attribute("style", format!("height:{}px;", height).as_str()).unwrap();
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().add_event_listener_with_callback("resize", window_resize_closure.as_ref().unchecked_ref()).unwrap();
        window_resize_closure.forget();
        

        view! { ctx, 
            Background()
            //ToggleMode()
            App()
        }
    });
}
