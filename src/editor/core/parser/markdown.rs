use adw::gdk::pango;
use adw::gdk::pango::FontDescription;
use gtk::prelude::{TextBufferExt, TextBufferExtManual, TextTagExt};
use gtk::PolicyType::Never;
use gtk::{TextBuffer, TextTag, TextTagTable, WrapMode};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::error::Error;
use std::fmt::{Debug, Display};

pub fn widget_from(markdown: &str) -> gtk::ScrolledWindow {
    // 解析 Markdown 文本
    let parser = Parser::new_ext(markdown, Options::all());

    // tag table
    let tag_table = TextTagTable::new();
    // set header tag
    let base_font_size = 32i32;
    let header_tags = vec!["h1", "h2", "h3", "h4", "h5", "h6"];
    for (idx, &header) in header_tags.iter().enumerate() {
        let font_size = base_font_size - (idx as i32 * 4);
        let header_tag = TextTag::new(Some(header));
        println!("tag name created: {:?}", header_tag.name().unwrap());
        let mut font_desc = FontDescription::new();
        font_desc.set_size(font_size * pango::SCALE);
        header_tag.set_font_desc(Some(&font_desc));
        tag_table.add(&header_tag);
    }

    // set emphasis tag
    let emphasis_tag = TextTag::new(Some("em_tag"));
    emphasis_tag.set_font(Some("Bold"));

    let buf = TextBuffer::new(Some(&tag_table));

    let document = gtk::TextView::builder()
        .vexpand(true)
        .wrap_mode(WrapMode::Word)
        .buffer(&buf)
        .build();

    let srcl = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(Never)
        .child(&document)
        .build();
    let mut flag = EventFlag::default();

    for event in parser {
        match event {
            Event::Start(tag) => flag.update(Some(&tag)),
            Event::End(tag_end) => {
                if tag_end == TagEnd::Paragraph {
                    let text = flag.accum_text.clone();
                    buf.insert(&mut buf.end_iter(), &text);
                    buf.insert(&mut buf.end_iter(), "\n");
                    flag.accum_text.clear();
                }

                if tag_end == TagEnd::Emphasis {
                    println!("end")
                }

                flag.update(None)
            }
            Event::Text(text) => {
                match flag.tag() {
                    None => flag.update(None),
                    Some(event_tag) => match event_tag {
                        Tag::Heading { level, .. } => {
                            let header_tag = format!("{}", level);
                            println!("tag name {}, header text: {}", header_tag, text);
                            buf.insert_with_tags_by_name(
                                &mut buf.end_iter(),
                                &text,
                                &[&header_tag],
                            );
                            buf.insert(&mut buf.end_iter(), "\n");
                        }
                        Tag::List(_) => {}
                        Tag::Item => {}
                        Tag::Paragraph => {                flag.accum_text.push_str(&text);
                            println!("text when text para event: {}", text);}
                        Tag::CodeBlock(_) => {}
                        Tag::HtmlBlock => {}
                        Tag::BlockQuote(_) => {}
                        Tag::FootnoteDefinition(_) => {}
                        Tag::Table(_) => {}
                        Tag::TableHead => {}
                        Tag::TableRow => {}
                        Tag::TableCell => {}
                        Tag::Emphasis => {
                            println!("em text: {}", text);
                            flag.accum_text.push_str(&text);
                            println!("text when text para event: {}", text);
                        }
                        Tag::Strong => {
                            println!("strong text: {}", text);
                            // buf.insert_with_tags(&mut buf.end_iter(), &text, &[&emphasis_tag])
                        }
                        Tag::Strikethrough => {}
                        Tag::Link { .. } => {}
                        Tag::Image { .. } => {}
                        Tag::MetadataBlock(_) => {}
                    },
                }

            }
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

    srcl
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
