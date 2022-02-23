use sycamore::prelude::RcSignal;
use crate::AppRoutes;


#[derive(Clone)]
pub struct DarkMode(pub RcSignal<bool>);

#[derive(Clone)]
pub struct CurrentRoute(pub RcSignal<AppRoutes>);

#[derive(Clone)]
pub struct LeftMenuOpened(pub RcSignal<bool>);


#[derive(Clone)]
pub struct BackgroundImage(pub RcSignal<String>);

#[derive(Clone)]
pub struct BackgroundVideo(pub RcSignal<String>);

