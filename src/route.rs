use sycamore_router::Route;


#[derive(Debug, Clone, Route)]
pub enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/bible")]
    Bible,
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
