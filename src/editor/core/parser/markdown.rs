use gtk::prelude::WidgetExt;
use gtk::Align::Start;
use gtk::{Align, NaturalWrapMode, PolicyType};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::fmt::{Debug, Display};
use gtk::pango::WrapMode;

pub fn widget_from(markdown: &str) -> gtk::ScrolledWindow {
    // 解析 Markdown 文本
    let parser = Parser::new_ext(markdown, Options::all());

    let document_box = gtk::ListBox::builder().hexpand(true).vexpand(true).build();

    let scrolled_window = gtk::ScrolledWindow::builder()
        //     .css_classes(["editor-scroller"])
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .child(&document_box)
        .build();

    // para
    let mut para_text = String::new();
    let mut is_para_text = false;
    let mut para_label = None;

    // header
    let mut header_text = String::new();
    let mut header_label = None;
    let mut is_header_text = false;

    for event in parser {
        match event {
            Event::Start(start_tag) => match start_tag {
                Tag::Paragraph => {
                    para_label = new_text_label();
                    is_para_text = true;
                }
                Tag::Heading { level, .. } => {
                    header_label = new_header_label();
                    if let Some(ref header_label) = header_label {
                        let css_name = format!("{}-label", level);
                        header_label.set_css_classes(&[&css_name]);
                        is_header_text = true;
                    }
                }
                Tag::Emphasis => {}
                Tag::Strong => {}
                Tag::BlockQuote(_) => {}
                Tag::CodeBlock(_) => {}
                Tag::List(_) => {}
                Tag::Item => {}
                _ => {}
            },
            Event::End(end_tag) => match end_tag {
                TagEnd::Paragraph => {
                    if let Some(para_label) = para_label.take() {
                        para_label.set_text(&para_text);
                        println!("############# lines: {}",para_label.max_width_chars());
                        if para_label.lines() == 1 {
                            para_label.set_halign(Start)
                        }
                        para_text.clear();
                        document_box.append(&para_label);
                        is_para_text = false
                    }
                }
                TagEnd::Heading(_) => {
                    if let Some(header_label) = header_label.take() {
                        header_label.set_text(&header_text);
                        header_text.clear();
                        document_box.append(&header_label);
                        is_header_text = false;
                    }
                }
                TagEnd::BlockQuote => {}
                TagEnd::CodeBlock => {}
                TagEnd::List(_) => {}
                TagEnd::Item => {}
                TagEnd::Emphasis => {}
                TagEnd::Strong => {}
                _ => {}
            },
            Event::Text(text) => {
                if is_para_text {
                    para_text.push_str(&text);
                } else if is_header_text {
                    header_text = text.to_string();
                }
            }
            Event::Code(code) => {}
            Event::InlineMath(_) => {}
            Event::DisplayMath(_) => {}
            Event::Html(_) => {}
            Event::InlineHtml(_) => {}
            Event::FootnoteReference(_) => {}
            Event::SoftBreak => {}
            Event::HardBreak => {}
            Event::Rule => {}
            Event::TaskListMarker(_) => {}
        }
    }

    scrolled_window
}

fn new_text_label() -> Option<gtk::Label> {
    Some(
        gtk::Label::builder()
            .wrap(true)
            .margin_start(24)
            .margin_end(24)
            .natural_wrap_mode(NaturalWrapMode::Word)
            .wrap_mode(WrapMode::Word)
            .use_markup(true)
            .hexpand(true)
            .vexpand(true)
            .build(),
    )
}

fn new_header_label() -> Option<gtk::Label> {
    Some(
        gtk::Label::builder()
            .hexpand(true)
            .margin_start(24)
            .margin_end(24)
            .halign(Start)
            .vexpand(true)
            .build(),
    )
}
