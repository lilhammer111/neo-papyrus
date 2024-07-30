use gtk::{Stack, StackPage, StackSidebar, StackSwitcher};

pub fn build_layout() {
    let exp_stack = Stack::builder().name("explore").build();
    let edi_stack = Stack::builder().name("editor").build();
    let switcher = StackSwitcher::builder().stack(&exp_stack).stack(&edi_stack).build();
}