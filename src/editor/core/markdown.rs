use pulldown_cmark::CowStr;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use header::handle_start;

pub fn parse(markdown: &str) -> CowStr {
    // 解析 Markdown 文本
    let parser = Parser::new_ext(markdown, Options::all());

    let mut pango_markup = String::new();
    let mut list_indent_level = 0;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    handle_start(level, &pango_markup);
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
            }
            Event::End(tag) => match tag {
                TagEnd::Heading(_) => {
                    pango_markup.push_str("</span>\n");
                }
                TagEnd::List(_) => {
                    list_indent_level = list_indent_level.saturating_sub(1);
                }
                _ => (),
            },
            _ => (),
        }
    }

    CowStr::from(pango_markup)
}

mod header {
    use pulldown_cmark::HeadingLevel;

    pub fn handle_start(level: HeadingLevel, mut pango_markup: &str) {
        let font_size = match level {
            HeadingLevel::H1 => "24pt", // Header 1 的字体大小
            HeadingLevel::H2 => "20pt", // Header 2 的字体大小
            HeadingLevel::H3 => "16pt", // Header 3 的字体大小
            _ => "14pt",                // 其他 Header 的字体大小
        };
        pango_markup.push_str(&format!("\n<span size='{}' weight='bold'>", font_size));
    }

    pub fn handle_end() {}

    pub fn handle_text() {}
}
