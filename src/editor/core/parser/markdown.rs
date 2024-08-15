use header::handle_start;
use pulldown_cmark::CowStr;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

pub fn parse(markdown: &str) -> CowStr {
    // 解析 Markdown 文本
    let parser = Parser::new_ext(markdown, Options::all());

    let mut pango_markup = String::new();
    let mut list_indent_level = 0;

    for event in parser {
        let fb = gtk::FlowBox::new();

        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    // handle_start(level, &mut pango_markup);
                    handle_start(level, &fb);
                }
                Tag::List(_) => {
                    list_indent_level += 1;
                }
                _ => (),
            },
            Event::Text(text) => {
                let indent = "    ".repeat(list_indent_level); // 每一级缩进
                if list_indent_level > 0 {
                    pango_markup.push_str(&format!(
                        "\n<span foreground='red'>{}• {}</span>\n",
                        indent, text
                    ));
                } else {
                    pango_markup.push_str(&text);
                }

                let header = gtk::Label::new(Some(&text));
                fb.append(&header);
            }
            Event::End(tag) => match tag {
                TagEnd::Heading(_) => {}
                TagEnd::List(_) => {}
                _ => (),
            },
            _ => (),
        }
    }

    CowStr::from(pango_markup)
}

mod header {
    use pulldown_cmark::HeadingLevel;

    pub fn handle_start(level: HeadingLevel, fb: &gtk::FlowBox) {
        // let font_size = match level {
        //     HeadingLevel::H1 => "24pt", // Header 1 的字体大小
        //     HeadingLevel::H2 => "20pt", // Header 2 的字体大小
        //     HeadingLevel::H3 => "16pt", // Header 3 的字体大小
        //     _ => "14pt",                // 其他 Header 的字体大小
        // };
        // pango_markup.push_str(&format!("\n<span size='{}' weight='bold'>", font_size));

        match level {
            HeadingLevel::H1 => {
                let prefix_label = gtk::Label::builder()
                    .css_name("prefix-h1")
                    .label("h1")
                    .visible(false)
                    .build();

                fb.append(&prefix_label);
            }
            HeadingLevel::H2 => {}
            HeadingLevel::H3 => {}
            _ => unreachable!(),
        };
    }

    #[allow(unused)]
    pub fn handle_end() {}

    #[allow(unused)]
    pub fn handle_text() {}
}
