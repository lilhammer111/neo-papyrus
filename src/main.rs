use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        view! {
            cx, p {"hello world"}
        }
    });
}
