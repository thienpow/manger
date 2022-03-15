use sycamore::prelude::*;
use crate::store::AppState;

#[component]
pub fn Account<G: Html>(ctx: Scope) -> View<G> {

    let _app_state = ctx.use_context::<AppState>();
    
    view! { ctx,
        div(class="content-section") {
            div(class="content-section-title") { "PROFILE DATA" }
            div(class="card-wrapper") {
                div(class="card") {
                    div(style="width:100%; display: flex; justify-content: center;") {
                        img(class="profile-img-big", loading="lazy", 
                            src="/assets/img/avatar_1.webp", 
                            alt="My Profile")
                    }

                    div(class="card__subtext") {
                        span(){"Display Name: "}
                    }
                    div(class="card-buttons") {
                        button(class="cbutton status-button") {"Edit"}
                    }
                }

                div(class="card") {

                    div(class="card__subtext") {
                        span(){"Real Name: "}
                        span(){"Email: "}
                        span(){"Phone: "}
                        div(class="row__seperator")
                        span(){"Age: "}
                        span(){"Gender: "}
                        span(){"Race: "}
                        span(){"Job: "}
                    }
                    div(class="card-buttons") {
                        button(class="cbutton status-button") {"Edit"}
                    }
                }
                div(class="card") {

                    div(class="card__subtext") {
                        span(){"Church Name: "}
                        span(){"Cell Group: "}
                        span(){"Other Info: "}
                        div(class="row__seperator")
                        span(){"Address: "}
                        span(){"City: "}
                        span(){"ZipCode: "}
                        span(){"State: "}
                        span(){"Country: "}
                    }
                    div(class="card-buttons") {
                        button(class="cbutton status-button") {"Edit"}
                    }
                }
            }

        }
    }
}
