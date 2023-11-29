use leptos::*;
use leptos::html::Input;
use leptos::logging::log;

use web_sys::{Document, Element, HtmlElement, Window, ScrollIntoViewOptions, ScrollBehavior};

use gloo::events::EventListener;

#[component]
fn ItemList(
    item_count: u8,
) -> impl IntoView {
    let key_input = create_node_ref::<Input>();

    let (highlighted_id, set_highlighted_id) = create_signal(0_i32);
    let (selected_id, set_selected_id) = create_signal(0_i32);
    let (key_pressed, set_key_pressed) = create_signal(String::from(""));

    let mut items: Vec<u8> = vec![];
    for id in 0..=item_count {
        items.push(id)
    }

    create_effect(move |_| {
        let listener = EventListener::new(&window(), "click", move |_| {
            // always set focus on main text input if user presses anywhere in window.
            if let Some(input) = key_input.get_untracked() {
                let _ = input.focus();
            }
        });

        // place focus on focus bar at start
        if let Some(input) = key_input.get() {
            request_animation_frame(move || {
                let _ = input.focus();
            });
        }

        on_cleanup(move || drop(listener));
    });

    // introduce derived signal modulo item_count and highlighted_id and selected
    let selected_list_id = move || (selected_id().rem_euclid((item_count + 1) as i32));
    let highlighted_list_id = move || (highlighted_id().rem_euclid((item_count + 1) as i32));

    view! {
        <div class="item-list">
            <input class="main-input" _ref=key_input type="text" on:keydown=move |ev| {
                set_key_pressed(ev.code().to_string());
                let code = ev.code();
                match code.as_ref() {
                    "ArrowDown" => {
                        set_highlighted_id.update(|n| *n += 1);
                        log!("down {:?}", highlighted_id());
                    },
                    "ArrowUp" => {
                        set_highlighted_id.update(|n| *n -= 1);
                        log!("up {:?}", highlighted_id());
                    },
                    "Enter" => {
                        set_selected_id(highlighted_id());
                        log!("enter {:?}", selected_id());
                    },
                    _ => {}
                }
            }/>
            <p>{key_pressed}</p>
        <p>{move || highlighted_id()}</p>
        {
            items
                .into_iter()
                .map(|id|
            {
                let is_highlighted = move || highlighted_list_id() as u8 == id;
                let is_selected = move || selected_list_id() as u8 == id;
                view!
                    {
                        <Item
                            is_highlighted=Signal::derive(is_highlighted)
                            is_selected=Signal::derive(is_selected)
                            id=id
                        />
                }
                })
                .collect_view()
        }
        </div>
    }
}

#[component]
fn Item(
    id: u8,
    #[prop(into)]
    is_highlighted: Signal<bool>,
    #[prop(into)]
    is_selected: Signal<bool>,
) -> impl IntoView {

    let css_id = format!("item--{id:?}");
    let css_id_clone = css_id.clone();

    create_effect(move |_| {
        if is_highlighted() {
            let elem = document().get_element_by_id(&css_id_clone);
            match elem {
                Some(elem) => elem.scroll_into_view_with_scroll_into_view_options(&ScrollIntoViewOptions::new().behavior(ScrollBehavior::Smooth)),
                None => {}
            }
        }
    });

    view! {
           <div class="item" id={css_id}
                class:highlighted=move || is_highlighted()
                class:selected=move || is_selected()
            />
}
}

#[component]
fn App() -> impl IntoView {

    let item_count: u8 = 5;

    view! {
        <div class="container">
            <h1>"selection toy example"</h1>
            <ItemList
                item_count=item_count
            />
            <button>"press"</button>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! {
        <App/>
    })
}
