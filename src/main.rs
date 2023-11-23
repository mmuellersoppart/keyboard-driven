use leptos::*;
use leptos::html::Input;

use gloo::events::EventListener;

#[component]
fn App() -> impl IntoView {
    let key_input = create_node_ref::<Input>();

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

    let (key_pressed, set_key_pressed) = create_signal(String::from(""));

    view! {

        <div class="input-container">
            <input class="main-input" _ref=key_input type="text" on:keydown=move |ev| {
                set_key_pressed(ev.code().to_string());
            }/>
            <p>{move || key_pressed}</p>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! {
        <App/>
    })
}
