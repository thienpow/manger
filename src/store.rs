
use sycamore::prelude::{RcSignal, create_rc_signal, ScopeRef};
use crate::route::{AppRoutes};
use crate::util;

pub fn initialize(ctx: ScopeRef) {

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    match window.local_storage().unwrap() {
        Some(local_storage) => {

            // check dark mode from media query. this will match wil user's pc/phone system setting
            let dark_mode_mq = window.match_media("(prefers-color-scheme: dark)").unwrap().unwrap().matches();
            // then, check it from local_storage
            let dark_mode_ls = local_storage.get_item("dark_mode").unwrap();
            // if got value from local_storage, we will use it, if none, then we use the value from media query
            let is_dark_mode = dark_mode_ls.as_deref() == Some("true") || (dark_mode_ls.is_none() && dark_mode_mq);

            if !is_dark_mode {
                document.body().unwrap().class_list().toggle("light-mode").expect("");
            }
            let dark_mode: RcSignal<bool> = create_rc_signal(is_dark_mode);

            let background_ls = match local_storage.get_item("background").unwrap() {
                Some(bg) => bg,
                None => "/assets/img/bg-cross.webp".to_string(),
            };

            let background: RcSignal<String> = create_rc_signal(background_ls);

            let inner_width: RcSignal<f64> = create_rc_signal(window.inner_width().unwrap().unchecked_into_f64());
            let inner_height: RcSignal<f64> = create_rc_signal(window.inner_height().unwrap().unchecked_into_f64());

            let app_state = AppState {
                dark_mode,
                background,
                inner_width, inner_height,
            };
            ctx.provide_context(app_state);
        
            let current_route = CurrentRoute(create_rc_signal(AppRoutes::Home));
            ctx.provide_context(current_route);

            
        },
        _ => {}
    }
    
}

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub dark_mode: RcSignal<bool>,
    pub background: RcSignal<String>,
    pub inner_width: RcSignal<f64>,
    pub inner_height: RcSignal<f64>,
}

impl AppState {

    pub fn toggle_dark_mode(&self) {
        self.dark_mode.set(!*self.dark_mode.get());

        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().class_list().toggle("light-mode").expect("");

        util::set_local_storage("dark_mode", self.dark_mode.get().to_string().as_str());
    }

    pub fn switch_background(&self, bg: &str) {
        self.background.set(bg.to_string());

        util::set_local_storage("background", bg);
    }

}

#[derive(Debug, Clone)]
pub struct CurrentRoute(pub RcSignal<AppRoutes>);
