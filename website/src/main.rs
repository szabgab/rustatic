use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Read;



fn main() {
    let source = read_file("../README.md");
    let content = markdown::to_html(&source);
    //println!("{}", content);
    let content = content.replace("<h1>", "<h1 class=\"title\">");
    let content = content.replace("<h2>", "<h2 class=\"title is-4\">");
    let content = content.replace("<h3>", "<h3 class=\"title is-5\">");

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse_file("template.html")
        .unwrap();

    let globals = liquid::object!({
        "title":   "rustatic",
        "content": content,
    });
    let html = template.render(&globals).unwrap();


    fs::create_dir_all("../_site").expect("could no create folder");
    let mut file = File::create("../_site/index.html").unwrap();
    writeln!(&mut file, "{}", html).unwrap();

}

fn read_file(filepath: &str) -> String{
    match File::open(filepath) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            return content;
        },
        Err(error) => {
            panic!("Error opening file {}: {}", filepath, error);
        },
    }
}
