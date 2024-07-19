use sycamore::{prelude::*, web::html::view};

fn main() {
    sycamore::render(|| view! {
            div(class="bg-red-300") { "hello world" }
});
}