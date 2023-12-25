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
        }
        class:red=move || count() % 2 == 1
        >


        //counter button
            // note: {count()} accesses the value of count once, and passes an i32 into the view, rendering it once, un-reactively.
            // note: {count} passes ina  function telling the framework to update the view everytime count changes.
            // same as this if you are on nightly
            // {move || count()}
        // text nodes are wrapped in quotation marks
            <p>"Count: " {count}</p>
        <br/>
        //progress bar
            //this is without implimenting a component
            /*<progress
                max="50"
                value=move || count() * 2
            />*/
        </button>
    <br/>
    // progress bar component
        <ProgressBar progress=count/>
    }
}

//add progress bar component
// define props by giving additional arguments to the component function
// you will need to add #[component] tag for any new components
#[component]
fn ProgressBar(progress: ReadSignal<i32>) -> impl IntoView {
    view! {
        <progress
            max="50"
            value=progress
        />
    }
}