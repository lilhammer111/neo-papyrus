use sycamore::prelude::*;

fn main() {
    sycamore::render(|ctx| {
        view! {ctx,
            div(class="bg-red-500 flex-center") {"hello world"}
        }
    });
}