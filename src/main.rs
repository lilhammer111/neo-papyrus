mod components;
mod ui;

use sycamore::prelude::*;
use ui::layout::RootLayout;
fn main() {
    sycamore::render(|ctx| -> View<DomNode> { 
        view!(
            ctx, 
            RootLayout()
        )
    })
}
