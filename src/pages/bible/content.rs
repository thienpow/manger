use std::{rc::Rc};
use sycamore::prelude::*;
use web_sys::{Event, Element};

use crate::{pages::bible::{store::{BibleState, VerseItem}, self}, store::AppState};
use crate::util;

#[derive(Clone)]
struct SelectionFirst(pub RcSignal<VerseItem>);


fn _get_marked_index(verse: &Rc<VerseItem>) -> (usize, usize) {

    if verse.book_id == 1 && verse.verse == 1 && verse.chapter == 1 {

        let x1 = 17;
        let x2 = 20;
        (x1, x2)
    } else {
        (0 ,0)
    }

}


#[component]
pub fn EmptyNavPanel<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="verse-content-nav-panel") {

        }
    }
}

#[component]
pub fn BackButton<G: Html>(cx: Scope) -> View<G> {
    let show_button = create_signal(cx, false);

    view! { cx,
        div(class="verse-content-nav-panel", style="border-top-left-radius: 12px;",
            on:mouseenter=move |_| show_button.set(true),
            on:mouseleave=move |_| show_button.set(false)
        ) {
            button(
                class=format!("verse-content-nav-button {}", if *show_button.get() && bible::util::check_if_not_first_page() {"nav-button-show"} else {""}),
                on:click=move |_| bible::util::scroll_to_previous_page(cx, 60)
            ) {
                i(class="icon-chevron-left")
            }
        }
    }
}

#[component]
pub fn NextButton<G: Html>(cx: Scope) -> View<G> {
    let show_button = create_signal(cx, false);

    view! { cx,
        div(class="verse-content-nav-panel", style="border-top-right-radius: 12px;",
            on:mouseenter=move |_| show_button.set(true),
            on:mouseleave=move |_| show_button.set(false)
        ) {
            button(
                class=format!("verse-content-nav-button {}", if *show_button.get() && bible::util::check_if_not_last_page() {"nav-button-show"} else {""}),
                on:click=move |_| bible::util::scroll_to_next_page(cx)
            ) {
                i(class="icon-chevron-right")
            }
        }
    }
}

#[component]
fn VerseItem<G: Html>(cx: Scope, verse: RcSignal<VerseItem>) -> View<G> {

    let bible_state = use_context::<BibleState>(cx);
    let verse =  verse.get();
    let prefix = format!("[{}:{}]  ", verse.chapter, verse.verse);

    let mut is_marked = false;
    let verse_text = verse.text.clone();
    let mut verse_text_before_mark: String = "".to_string();
    let mut verse_text_with_mark: String = "".to_string();
    let mut verse_text_after_mark: String = "".to_string();

    /*
    
    let (x1, x2) = get_marked_index(&verse);
    if x2 > 0 {
        is_marked = true;

        verse_text_before_mark = match str::get(verse.text.as_str(), 0..x1) {
            Some(result) => result.to_string(),
            None => "".to_string()
        };
        verse_text_with_mark = match str::get(verse.text.as_str(), x1..x2) {
            Some(result) => result.to_string(),
            None => "".to_string()
        };
        verse_text_after_mark = match str::get(verse.text.as_str(), x2..) {
            Some(result) => result.to_string(),
            None => "".to_string()
        };
    }
     */

    let on_mousedown = move |_: Event| {
        bible_state.selection_first_verse.set(VerseItem{
            //id: verse.id,
            book_id: verse.book_id,
            chapter: verse.chapter,
            verse: verse.verse,
            text: verse.text.clone()
        });
    };
    
    view! { cx,
        p(on:mousedown=on_mousedown
        ) {
            span(class="", style="-webkit-user-select:none; user-select:none;") {(prefix)}
            (if is_marked {
                view! {cx,
                    span(){(verse_text_before_mark)}
                    mark() {
                        span(){(verse_text_with_mark)}
                    }
                    span(){(verse_text_after_mark)}
                }
            } else {
                view! {cx, span(){(verse_text)}}
            })
        }
    }
}


#[component]
pub fn Content<G: Html>(cx: Scope) -> View<G> {

    let app_state = use_context::<AppState>(cx);
    let bible_state = use_context::<BibleState>(cx);
    let verse_content = create_node_ref(cx);

    let get_filtered_verses = create_memo(cx, || {
        bible_state
            .display_verses
            .get()
            .iter()
            //.filter(|v| bible_state.selected_bible_chapter.get().id == v.get().chapter)
            .cloned()
            .collect::<Vec<_>>()
    });

    let get_content_style = create_memo(cx, || {
        if *bible_state.is_bookview.get() {
            "bible-verse-content-bookview"
        } else {
            "bible-verse-content-vscrollview"
        }
    });

    let on_click = move |_e: Event| {
        if *app_state.inner_width.get() <= 420.0 {
            bible_state.show_bible_toc.set(false);
            bible_state.pin_bible_toc.set(false);
        }
    };

    let _on_mouseup = move |_: Event| {
        let window = web_sys::window().unwrap();
        let selected_text = window.get_selection().unwrap().unwrap().to_string().as_string().unwrap();
        
        if !selected_text.is_empty() {
            //let mut found_index = 0;
            let splited = selected_text.split("\n").into_iter().clone();
            
            for (_i, sel_text) in splited.into_iter().enumerate() {
    
                /* do not use... just keep for reference for debugging
                if i == 2 {
                    match sel_text.chars().position(|c| c == ']') {
                        Some(num) => {
                            found_index = num as u32;
                        },
                        _ => {
                            found_index = -1
                        }
                    }
                    
                }
                 */

                if !sel_text.is_empty() {
                    //console::log_1(&format!("index: {}, text: {}", i, sel_text).into());
                }
            }

            match bible_state.selection_first_verse.get() {
                verse_item => {
                    
                    let arr = js_sys::Array::new();
                    arr.push(&util::js_array(&["book_id", format!("{}", verse_item.book_id).as_str()]));
                    arr.push(&util::js_array(&["chapter", format!("{}", verse_item.chapter).as_str()]));
                    arr.push(&util::js_array(&["First verse selected", format!("{}", verse_item.verse).as_str()]));
                     
                    //console::table_1(&arr);

                }
            };
            
        }


    };

    let on_touchend = move |_: Event| {
        let bible_state = use_context::<BibleState>(cx);
        if !*bible_state.is_bookview.get() {
            return;
        }

        let e = verse_content.get::<DomNode>().unchecked_into::<Element>();
        let previous_scroll_page = (*bible_state.current_verse_scroll_x.get() / (e.client_width() as f64 + 48.0)).floor();
        let current_scroll_page_actual = e.scroll_left() as f64 / (e.client_width() as f64 + 48.0);
        
        let mut page_to_scroll = *bible_state.current_verse_page.get() as f64;
        let current_scroll_page_diff = current_scroll_page_actual - previous_scroll_page;
        if current_scroll_page_diff > 1.2/10.0 {
            page_to_scroll = previous_scroll_page + current_scroll_page_diff.ceil();
        } else if current_scroll_page_diff < -1.2/10.0 {
            page_to_scroll = previous_scroll_page + current_scroll_page_diff.floor();
        }
        
        //console::log_1(&format!("{}", page_to_scroll).into());
        bible_state.current_verse_page.set(page_to_scroll as u32);
        let x = (page_to_scroll * e.client_width() as f64) as f64 + (48.0 * page_to_scroll as f64);
        bible_state.current_verse_scroll_x.set(x);
        e.scroll_with_x_and_y(x, 0.0);
    };


    let bible_content_style = create_memo(cx,|| {
        let inner_width: f64 = *app_state.inner_width.get();
        let inner_height: f64 = *app_state.inner_height.get();
        let mut height =  inner_height-116.0;
        if inner_width <= 738.0 {
            height =  inner_height-128.0;
        } 
        let mut width = inner_width;
        if *bible_state.show_bible_toc.get() || *bible_state.pin_bible_toc.get() {
            if !*bible_state.toc_animating.get() {
                width = inner_width-188.0;
            }
        }

        format!("width:{}px; height:{}px;", width, height)
    });

    let verse_content_style = create_memo(cx,|| {
        let inner_width: f64 = *app_state.inner_width.get();
        let inner_height: f64 = *app_state.inner_height.get();
        let mut height =  inner_height-116.0-50.0;
        if inner_width <= 738.0 {
            height =  inner_height-128.0-50.0;
        } 
        
        format!("font-size:{}pt;height:{}px;min-height:{}px;max-height:{}px;", *bible_state.verse_text_size.get(), height, height, height)
    });

    view! { cx,

        article(
            class="bible-content",
            style=*bible_content_style.get()
        ) {
            //scroll_to_previous_page
            (
                if *bible_state.is_bookview.get() {
                    BackButton(cx, ())
                } else {
                    EmptyNavPanel(cx, ())
                }
            )
            
            //(*key_code.get())
            
            div(
                class="bible-verse-wrapper",
                style=*bible_content_style.get()
            ) {
                div(ref=verse_content,
                    id="bible-verse-content", 
                    class=*get_content_style.get(),
                    on:click=on_click,
                    //on:mouseup=on_mouseup,
                    on:touchend=on_touchend,
                    style=*verse_content_style.get()
                ) {
                    Keyed {
                        iterable: get_filtered_verses,
                        view: |cx, verse| view! { cx,
                            VerseItem(verse)
                        },
                        key: |verse| verse.get().verse,
                    }
                }
            }

            //scroll_to_next_page
            (
                if *bible_state.is_bookview.get() {
                    NextButton(cx, ())
                } else {
                    EmptyNavPanel(cx, ())
                }
            )
        
        }

    }
}