

use sycamore::prelude::*;

use crate::route::AppRoutes;
use crate::store::{CurrentRoute};
use crate::svg::{LOGO_SVG, NOTIF_SVG};

#[component]
pub fn Header<G: Html>(ctx: Scope) -> View<G> {

    let CurrentRoute(current_route) = ctx.use_context::<CurrentRoute>();

    let header_ref = ctx.create_node_ref();
    let left_menu_ref = ctx.create_node_ref();

    let on_search_focus = move |_| {
        header_ref.get::<DomNode>().add_class("wide");
    };
    let on_search_blur = move |_| {
        header_ref.get::<DomNode>().remove_class("wide");
    };

    view! { ctx,

        header(ref=header_ref, id="header", class="header") {
            div(ref=left_menu_ref, class="header-menu-left") {
                a(class="logo", aria-label="Manger Home Page", href="/", dangerously_set_inner_html=LOGO_SVG) 
            }
            div(class="menu-gap")
            div(class="header-menu") {
                a(class=(match *current_route.get() {AppRoutes::Home => "navbar-menu-home is-active", _ => "navbar-menu-home"}), href="/") {"Home"}
                a(class=(match *current_route.get() {AppRoutes::Bible => "is-active", _ => ""}), href="/bible") {"Bible Study"}
                a(class=(match *current_route.get() {AppRoutes::Community => "is-active", _ => ""}), href="/community") {"Community"}
            }
            
            div(class="search-bar") {
                input(type="text", placeholder="Search", on:focus=on_search_focus, on:blur=on_search_blur)
            }
            div(class="menu-gap")
            div(class="header-profile") {
                div(class="notification") {
                    span(class="notification-number") {(10)}
                    span(dangerously_set_inner_html=NOTIF_SVG) 
                }
                div(class="icon-gap")
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