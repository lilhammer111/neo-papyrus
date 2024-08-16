use adw::gdk::pango;
use adw::gdk::pango::FontDescription;
use gtk::prelude::{TextBufferExt, TextBufferExtManual, TextTagExt};
use gtk::Justification::Center;
use gtk::PolicyType::Never;
use gtk::{TextBuffer, TextTag, TextTagTable, WrapMode};
use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
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

    // set strong tag
    let strong_tag = TextTag::new(Some("strong_tag"));
    strong_tag.set_font(Some("Bold"));
    tag_table.add(&strong_tag);

    let strong_tagname = strong_tag.name().unwrap().to_string();

    // set emphasis tag
    let emphasis_tag = TextTag::new(Some("em_tag"));
    emphasis_tag.set_font(Some("Italic"));
    tag_table.add(&emphasis_tag);

    let emphasis_tagname = emphasis_tag.name().unwrap().to_string();

    // set inline code tag
    let inline_code_tag = TextTag::new(Some("inline_code"));
    inline_code_tag.set_background(Some("#f0f4f8"));
    inline_code_tag.set_background_full_height(false);
    inline_code_tag.set_left_margin(2);
    inline_code_tag.set_right_margin(2);
    inline_code_tag.set_justification(Center);
    inline_code_tag.set_rise(4 * pango::SCALE);
    tag_table.add(&inline_code_tag);

    let inline_code_tagname = inline_code_tag.name().unwrap().to_string();

    // create buffer
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

    let mut tags: Vec<String> = Vec::new();
    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {}
                Tag::Heading { level, .. } => {
                    let header_tag = format!("{}", level);
                    tags.push(header_tag);
                }
                Tag::List(_) => {}
                Tag::Item => {
                    buf.insert(&mut buf.end_iter(), "·");
                }
                Tag::Strong => tags.push(strong_tagname.clone()),
                Tag::Emphasis => tags.push(emphasis_tagname.clone()),
                Tag::BlockQuote(_) => {}
                Tag::CodeBlock(_) => {}
                Tag::HtmlBlock => {}
                Tag::FootnoteDefinition(_) => {}
                Tag::Table(_) => {}
                Tag::TableHead => {}
                Tag::TableRow => {}
                Tag::TableCell => {}
                Tag::Strikethrough => {}
                Tag::Link { .. } => {}
                Tag::Image { .. } => {}
                Tag::MetadataBlock(_) => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Paragraph => {
                    buf.insert(&mut buf.end_iter(), "\n");
                }
                TagEnd::Heading(level) => {
                    tags.retain(|tag| tag != format!("{}", level).as_str());
                    buf.insert(&mut buf.end_iter(), "\n");
                }
                TagEnd::Strong => {
                    tags.retain(|tag| tag != strong_tagname.as_str());
                }
                TagEnd::Emphasis => {
                    tags.retain(|tag| tag != emphasis_tagname.as_str());
                }
                TagEnd::BlockQuote => {}
                TagEnd::CodeBlock => {}
                TagEnd::HtmlBlock => {}
                TagEnd::List(_) => {}
                TagEnd::Item => {
                    buf.insert(&mut buf.end_iter(), "\n");
                }
                TagEnd::FootnoteDefinition => {}
                TagEnd::Table => {}
                TagEnd::TableHead => {}
                TagEnd::TableRow => {}
                TagEnd::TableCell => {}
                TagEnd::Strikethrough => {}
                TagEnd::Link => {}
                TagEnd::Image => {}
                TagEnd::MetadataBlock(_) => {}
            },
            Event::Text(text) => {
                let tags = tags.iter().map(|tag| tag.as_str()).collect::<Vec<&str>>();
                println!("tags: {:?}", tags);
                buf.insert_with_tags_by_name(&mut buf.end_iter(), &text, &tags);
            }
            Event::Code(code) => {
                tags.push(inline_code_tagname.clone());
                let tags_slice = tags.iter().map(|tag| tag.as_str()).collect::<Vec<&str>>();
                buf.insert_with_tags_by_name(&mut buf.end_iter(), &code, &tags_slice);
                tags.retain(|tag| tag != inline_code_tagname.as_str());
            }
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
