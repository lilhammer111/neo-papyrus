use adw::gdk::pango;
use adw::gdk::pango::{AttrList, WrapMode};
use gtk::prelude::BoxExt;
use gtk::Align::{Center, Start};
use gtk::Orientation::Horizontal;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::error::Error;
use std::fmt::{Debug, Display};

pub fn widget_from(markdown: &str) -> gtk::ListBox {
    // 解析 Markdown 文本
    let parser = Parser::new_ext(markdown, Options::all());

    let document_box = gtk::ListBox::builder().hexpand(true).build();
    let mut flag = EventFlag::default();

    for event in parser {
        match event {
            Event::Start(tag) => {
                if let Tag::List(_) = tag {
                    let listbox = gtk::ListBox::builder().build();
                    flag.list_stack.push(listbox);
                }
                flag.update(Some(&tag))
            }
            Event::End(tag_end) => {
                if let TagEnd::List(_is_order) = tag_end {
                    document_box.append(flag.list_stack.last().unwrap());
                    flag.list_stack.remove(flag.list_stack.len() - 1);
                }
                flag.update(None)
            }
            Event::Text(text) => match flag.tag() {
                None => flag.update(None),
                Some(tag) => match tag {
                    Tag::Paragraph => {
                        let para = gtk::Label::builder()
                            .css_classes(["paragraph"])
                            .halign(Start)
                            .label(text.as_ref())
                            .margin_top(4)
                            .margin_bottom(4)
                            .margin_start(24)
                            .margin_end(24)
                            .wrap(true)
                            // .natural_wrap_mode(NaturalWrapMode::Word)
                            .wrap_mode(WrapMode::Word)
                            .build();
                        document_box.append(&para)
                    }
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
                    Tag::BlockQuote(_) => {}
                    Tag::CodeBlock(_) => {}
                    Tag::HtmlBlock => {}
                    Tag::List(_) => {}
                    Tag::Item => {
                        let bullet_label = gtk::Label::builder()
                            .label("●")
                            .margin_start(24)
                            .margin_end(6)
                            .build();

                        // 调整符号的字体大小
                        let mut attr_list = AttrList::new();
                        attr_list.insert(pango::AttrSize::new_size_absolute(6 * pango::SCALE)); // 调整大小
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

    document_box
}

struct EventFlag<'a> {
    tag_kind: Option<Tag<'a>>,
    indent_level: u8,
    header_level: HeadingLevel,
    list_stack: Vec<gtk::ListBox>,
}

impl Default for EventFlag<'_> {
    fn default() -> Self {
        Self {
            tag_kind: None,
            indent_level: 0,
            header_level: HeadingLevel::H1,
            list_stack: vec![],
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
