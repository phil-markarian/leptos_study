use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button // define an event listener with on:
        on:click=move |_| {
            set_count.update(|n| *n += 1)
        }>

            // text nodes are wrapped in quotation marks
            "Click me: " // {move || count()}
            // same as this if you are on nightly
            {count}
        // note: {count()} accesses the value of count once, and passes an i32 into the view, rendering it once, un-reactively.
        // note: {count} passes ina  function telling the framework to update the view everytime count changes.
        </button>
    }
}