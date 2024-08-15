use gtk::prelude::{BoxExt, TextBufferExt, TextViewExt};
use gtk::Align::{Center, Start};
use gtk::Orientation::Horizontal;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

pub fn widget_from(markdown: &str) -> gtk::ListBox {
    // 解析 Markdown 文本
    let parser = Parser::new_ext(markdown, Options::all());

    let document_box = gtk::ListBox::new();

    let mut header_box: Option<HeaderBox> = None;
    let mut para_box: Option<gtk::TextView> = None;
    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    let row = gtk::TextView::builder()
                        .margin_top(5)
                        .margin_bottom(5)
                        .margin_start(24)
                        .build();
                    para_box = Some(row)
                }
                Tag::Heading { level, .. } => {
                    let row = gtk::Box::builder()
                        .orientation(Horizontal)
                        .valign(Center)
                        .build();
                    HeaderBox::handle_start(level, &row);
                    header_box = Some(HeaderBox::new(row, level as u8));
                }
                Tag::BlockQuote(_) => {}
                Tag::CodeBlock(_) => {}
                Tag::HtmlBlock => {}
                Tag::List(_) => {}
                Tag::Item => {}
                Tag::FootnoteDefinition(_) => {}
                Tag::Table(_) => {}
                Tag::TableHead => {}
                Tag::TableRow => {}
                Tag::TableCell => {}
                Tag::Emphasis => {}
                Tag::Strong => {
                    let _bold_label = gtk::Label::builder().css_classes(["bold"]).build();
                }
                Tag::Strikethrough => {}
                Tag::Link { .. } => {}
                Tag::Image { .. } => {}
                Tag::MetadataBlock(_) => {}
            },
            Event::Text(text) => {
                // println!("text: {text}");
                if let Some(HeaderBox { ref row, level }) = header_box {
                    let header_label = gtk::Label::builder()
                        .css_classes([
                            format!("h{}-label", level.to_string()),
                            "header-label".to_string(),
                        ])
                        .valign(Center)
                        .label(&*text)
                        .build();
                    row.append(&header_label)
                }

                if let Some(ref row) = para_box {
                    let buf = row.buffer();
                    buf.set_text(text.as_ref())
                }
            }
            Event::End(tag) => match tag {
                TagEnd::Heading(_) => {
                    if let Some(HeaderBox { ref row, .. }) = header_box.take() {
                        document_box.append(row);
                    }
                }
                TagEnd::Paragraph => {
                    if let Some(row) = para_box.take() {
                        document_box.append(&row);
                    }
                }
                _ => unreachable!(),
            },
            // Event::End(_) => {
            //     if let Some(HeaderBox { ref row, .. }) = header_box.take() {
            //         document_box.append(row);
            //     }
            // }
            _ => (),
        }
    }

    document_box
}

pub struct EmphasisBox {}

pub struct Paragraph {}

impl Paragraph {}

pub struct HeaderBox {
    pub row: gtk::Box,
    pub level: u8,
}

impl HeaderBox {
    pub fn new(row: gtk::Box, level: u8) -> Self {
        Self { row, level }
    }

    pub fn handle_start(level: HeadingLevel, fb: &gtk::Box) {
        let prefix_label = gtk::Label::builder()
            .vexpand_set(false)
            .css_classes(["header-prefix"])
            .valign(Center)
            .halign(Center)
            .width_request(20)
            .margin_end(4)
            // .visible(false) // todo
            .build();

        match level {
            HeadingLevel::H1 => {
                prefix_label.set_label("h1");
            }
            HeadingLevel::H2 => {
                prefix_label.set_label("h2");
            }
            HeadingLevel::H3 => {
                prefix_label.set_label("h3");
            }
            HeadingLevel::H4 => {
                prefix_label.set_label("h4");
            }
            HeadingLevel::H5 => {
                prefix_label.set_label("h5");
            }
            HeadingLevel::H6 => {
                prefix_label.set_label("h6");
            }
        };
        fb.append(&prefix_label);
    }

    #[allow(unused)]
    pub fn handle_end() {}

    #[allow(unused)]
    pub fn handle_text() {}
}
