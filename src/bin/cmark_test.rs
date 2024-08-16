use pulldown_cmark::{Event, Options, Parser};

fn main() {
    let md = "### header 1  \n*如果*你的**Markdown**文本中有某些不规范的换行符或空白字符，可能会导致 `pulldown_cmark` 解析时错误地将文本分成多个段落。Markdown 对换行符和空行有特定的处理方式，多个连续的换行符或一个空行通常表示段落的分割。";

    let parser = Parser::new_ext(md, Options::all());

    for event in parser {
        match event {
            Event::Start(tag) => {
                println!("tag {:?}", tag);
            }
            Event::End(end_tag) => {
                println!("tag end {:?}", end_tag);
            }
            Event::Text(text) => {
                println!("{}", text)
            }
            _ => {}
        }
    }
}
