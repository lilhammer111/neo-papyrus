mod components;
mod ui;

use sycamore::prelude::*;
use ui::layout::Layout;

fn main() {
    sycamore::render(|ctx| -> View<DomNode> { 
        view!(
            ctx, 
            Layout()
        )
    })
}
