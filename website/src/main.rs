use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;


fn main() {
    let content = markdown::file_to_html(Path::new("../README.md")).unwrap();
    //println!("{}", content);

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


