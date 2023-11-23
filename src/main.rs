use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <p>"Hello, world!"</p>
    }
}

fn main() {
    mount_to_body(|| view! {
        <App/>
    })
}
