use adw::gdk::pango;
use adw::gdk::pango::{AttrList, WrapMode};
use gtk::prelude::{BoxExt, TextBufferExt, TextViewExt, WidgetExt};
use gtk::Align::{Center, Start};
use gtk::Orientation::{Horizontal, };
use gtk::{PolicyType, TextBuffer};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::error::Error;
use std::fmt::{Debug, Display};
use gtk::ScrollablePolicy::{Minimum, Natural};

pub fn widget_from(markdown: &str) -> gtk::ScrolledWindow {
    // 解析 Markdown 文本
    let parser = Parser::new_ext(markdown, Options::all());

    let document_box = gtk::ListBox::builder()
        .hexpand(true)
        .vexpand(true)
        .build();
    // let tv_scroller = gtk::ScrolledWindow::builder()
    //     .css_classes(["editor-scroller"])
    //     .vscrollbar_policy(PolicyType::Automatic)
    //     .hscrollbar_policy(PolicyType::Never)
    //     .child(&document_box)
    //     .build();

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&document_box)
        .build();

    let mut flag = EventFlag::default();

    for event in parser {
        match event {
            Event::Start(tag) => {
                if let Tag::List(_) = tag {
                    let listbox = gtk::ListBox::builder().build();
                    flag.list_stack.push(listbox);
                }
                println!(
                    "########################################## start tag: {:?}\n",
                    tag
                );
                if tag == Tag::Paragraph {
                    let textview = gtk::TextView::builder()
                        .margin_top(4)
                        .margin_bottom(4)
                        .margin_start(24)
                        .margin_end(24)
                        .css_classes(["paragraph"])
                        .editable(false)
                        .vexpand(true)
                        .hexpand(true)
                        .wrap_mode(gtk::WrapMode::Word)
                        .build();
                    flag.textview = Some(textview)
                }

                flag.update(Some(&tag))
            }
            Event::End(tag_end) => {
                if let TagEnd::List(_is_order) = tag_end {
                    document_box.append(flag.list_stack.last().unwrap());
                    flag.list_stack.remove(flag.list_stack.len() - 1);
                }

                if tag_end == TagEnd::Paragraph {
                    let para_text = flag.accum_text.clone();
                    if let Some(textview) = flag.textview.take() {
                        let buf = TextBuffer::builder()
                            .text(&para_text)
                            .build();

                        textview.set_buffer(Some(&buf));
                        document_box.append(&textview);
                        flag.accum_text.clear()
                    }
                }

                flag.update(None)
            }
            Event::Text(text) => match flag.tag() {
                None => flag.update(None),
                Some(tag) => match tag {
                    Tag::Heading { level, .. } => {
                        let header_box = gtk::Box::builder()
                            .orientation(Horizontal)
                            .valign(Center)
                            .build();
                        let header_label = gtk::Label::builder()
                            .css_classes([format!("{}-label", level), "header-label".to_string()])
                            .valign(Center)
                            .label(text.as_ref())
                            .build();
                        let prefix_label = gtk::Label::builder()
                            .vexpand_set(false)
                            .label(format!("{}", level))
                            .css_classes(["header-prefix"])
                            .valign(Center)
                            .halign(Center)
                            .width_request(20)
                            .margin_end(4)
                            // .visible(false) // todo
                            .build();
                        header_box.append(&prefix_label);
                        header_box.append(&header_label);

                        document_box.append(&header_box);
                    }

                    Tag::List(_) => {}
                    Tag::Item => {
                        let bullet_label = gtk::Label::builder()
                            .label("●")
                            .margin_start(24)
                            .margin_end(6)
                            .build();

                        // 调整符号的字体大小
                        let mut attr_list = AttrList::new();
                        attr_list.insert(pango::AttrSize::new_size_absolute(8 * pango::SCALE)); // 调整大小
                        bullet_label.set_attributes(Some(&attr_list));

                        let item_label = gtk::Label::builder()
                            .css_classes(["item"])
                            .halign(Start)
                            .label(text.as_ref())
                            .margin_top(4)
                            .margin_bottom(4)
                            .margin_end(24)
                            .wrap(true)
                            .wrap_mode(WrapMode::Word)
                            .build();

                        let item_box = gtk::Box::builder().orientation(Horizontal).build();

                        item_box.append(&bullet_label);
                        item_box.append(&item_label);

                        flag.list_stack.last().unwrap().append(&item_box);
                    }
                    Tag::Paragraph => {
                        println!(
                            "current tag: {:?}， unit text: {}\n",
                            flag.tag().unwrap(),
                            text,
                        );
                        flag.accum_text.push_str(&text);
                        println!("accu para: {}\n", flag.accum_text)
                    }
                    Tag::CodeBlock(_) => {}
                    Tag::HtmlBlock => {}
                    Tag::BlockQuote(_) => {}
                    Tag::FootnoteDefinition(_) => {}
                    Tag::Table(_) => {}
                    Tag::TableHead => {}
                    Tag::TableRow => {}
                    Tag::TableCell => {}
                    Tag::Emphasis => {}
                    Tag::Strong => {}
                    Tag::Strikethrough => {}
                    Tag::Link { .. } => {}
                    Tag::Image { .. } => {}
                    Tag::MetadataBlock(_) => {}
                },
            },
            Event::Code(_) => {}
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

struct EventFlag<'a> {
    tag_kind: Option<Tag<'a>>,
    indent_level: u8,
    header_level: HeadingLevel,
    list_stack: Vec<gtk::ListBox>,
    accum_text: String,
    textview: Option<gtk::TextView>,
}

impl Default for EventFlag<'_> {
    fn default() -> Self {
        // let para = gtk::Label::builder()
        //     .css_classes(["paragraph"])
        //     .label(para_text)
        //     .halign(Start)
        //     .margin_top(4)
        //     .margin_bottom(4)
        //     .margin_start(24)
        //     .margin_end(24)
        //     .wrap(true)
        //     // .natural_wrap_mode(NaturalWrapMode::Word)
        //     .wrap_mode(WrapMode::Word)
        //     .build();

        Self {
            tag_kind: None,
            indent_level: 0,
            header_level: HeadingLevel::H1,
            list_stack: vec![],
            accum_text: String::new(),
            textview: None,
        }
    }
}

impl<'a> EventFlag<'a> {
    fn update(&mut self, tag: Option<&Tag<'a>>) {
        if let Some(t) = tag {
            self.tag_kind = Some(t.clone());
        }
    }

    fn tag(&self) -> Option<Tag> {
        self.tag_kind.clone()
    }

    // fn header_level(&mut self, level: HeadingLevel) -> Result<(), KindFlagError> {
    //     if let Some(tag ) = self.tag_kind {
    //         if tag == {
    //             self.header_level = level;
    //             Ok(())
    //         }
    //     } else {
    //         let e = KindFlagError::InvalidOperation(
    //             "Header level can only be set for Heading tags.".to_string(),
    //         );
    //         Err(e)
    //     }
    // }
}

#[derive(Debug, Clone)]
pub enum KindFlagError {
    InvalidOperation(String),
    // 可以添加更多错误类型
}

impl Error for KindFlagError {}

impl Display for KindFlagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KindFlagError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}
