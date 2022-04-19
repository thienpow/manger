use sycamore::prelude::*;

use crate::components::safearea::SafeArea;
use crate::route::AppRoutes;
use crate::store::CurrentRoute;
use crate::svg::NOTIF_SVG;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    let CurrentRoute(current_route) = use_context::<CurrentRoute>(cx);

    let header_ref = create_node_ref(cx);
    let left_menu_ref = create_node_ref(cx);

    let on_search_focus = move |_| {
        header_ref.get::<DomNode>().add_class("wide");
    };
    let on_search_blur = move |_| {
        header_ref.get::<DomNode>().remove_class("wide");
    };

    view! { cx,
        SafeArea {}

        header(ref=header_ref, id="header", class="header") {
            div(ref=left_menu_ref, class="header-menu-left") {
                a(class="logo", aria-label="Manger Home Page", href="/") {
                    img(loading="lazy", alt="Manger Logo", src="/assets/img/logo-192.png", width="100%")
                }
            }
            div(class="menu-gap")
            div(class="header-menu") {
                a(class=(match *current_route.get() {AppRoutes::Home => "navbar-menu-home is-active", _ => "navbar-menu-home"}), href="/") {"Home"}
                a(class=(match *current_route.get() {AppRoutes::Bible => "is-active", _ => ""}), href="/bible") {"Bible"}
                a(class=(match *current_route.get() {AppRoutes::Community => "is-active", _ => ""}), href="/community") {"Community"}
            }

            div(class="search-bar") {
                input(type="text", placeholder="Search", on:focus=on_search_focus, on:blur=on_search_blur)
            }
            div(class="menu-gap")
            div(class="header-profile") {
                div(class="notification") {
                    span(dangerously_set_inner_html=NOTIF_SVG)
                    span(class="notification-number") {(10)}
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
