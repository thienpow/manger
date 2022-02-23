

use sycamore::prelude::*;

use crate::AppRoutes;
use crate::context::{CurrentRoute};
use crate::svg::{LOGO_SVG, NOTIF_SVG};

#[component]
pub fn Header<G: Html>(ctx: ScopeRef) -> View<G> {

    let CurrentRoute(current_route) = ctx.use_context::<CurrentRoute>();
    //let LeftMenuOpened(left_menu_opened) = ctx.use_context::<LeftMenuOpened>();

    let header_ref = ctx.create_node_ref();
    let left_menu_ref = ctx.create_node_ref();

    let handle_search_focus = move |_| {
        //let header = web_sys::window().unwrap().document().unwrap().get_element_by_id("header").unwrap();
        //header.class_list().set_value("header wide");
        header_ref.get::<DomNode>().add_class("wide");
    };
    let handle_search_blur = move |_| {
        //let header = web_sys::window().unwrap().document().unwrap().get_element_by_id("header").unwrap();
        //header.class_list().set_value("header");
        header_ref.get::<DomNode>().remove_class("wide");
    };

    /*
    let handle_menu_click = move |_| {
        left_menu_opened.set(!*left_menu_opened.get());
        if *left_menu_opened.get() {
            left_menu_ref.get::<DomNode>().add_class("menu-left-opened"); 
        } else {
            left_menu_ref.get::<DomNode>().remove_class("menu-left-opened"); 
        }
    };
    */

    view! { ctx,

        div(ref=header_ref, id="header", class="header") {
            div(ref=left_menu_ref, class="menu-left") {
                a(class="logo", aria-label="Manger Home Page", href="/", dangerously_set_inner_html=LOGO_SVG) 
            }
            
            div(class="header-menu") {
                a(class=(match *current_route.get() {AppRoutes::Home => "is-active", _ => ""}), href="/") {"Home"}
                a(class=(match *current_route.get() {AppRoutes::BibleStudy => "is-active", _ => ""}), href="/bible") {"Bible Study"}
                a(class=(match *current_route.get() {AppRoutes::Community => "is-active", _ => ""}), href="/community") {"Community"}
            }
            
            div(class="search-bar") {
                input(type="text", placeholder="Search", on:focus=handle_search_focus, on:blur=handle_search_blur)
            }

            /*
            SearchBar(SearchBar{
                focus: Box::new(handle_search_focus),
                blur: Box::new(handle_search_blur)
            })
            */

            div(class="header-profile") {
                div(class="notification") {
                    span(class="notification-number") {(3)}
                    span(dangerously_set_inner_html=NOTIF_SVG) 
                }
                div() {
                    a(class=(match *current_route.get() {AppRoutes::Profile => "is-active", _ => ""}), href="/profile", alt="Profile") {
                        img(class="profile-img", loading="lazy",
                        src="/assets/img/avatar_1.webp", 
                        alt="My Profile")
                    }
                }
            }
        }
    }
}