use std::fs::File;
use std::io::Write;
use elasticlunr::{Index, Language};

fn main() {
//    let mut index = Index::new(&["title", "body"]);
    let mut index = Index::with_language(Language::Japanese, &["title", "body"]);
    index.add_doc("1", &["This is a title", "This is body text!"]);
    index.add_doc("2", &["日本語のタイトルです。", "日本語の本文です。"]);
    index.add_doc("3", &["日本語とEnglishが混在mixした表題titleです。", "日本語とEnglishが混在mixした本文bodyです。"]);
    let mut file = File::create("out.json").unwrap();
    file.write_all(index.to_json_pretty().as_bytes());
//    println!(index.to_json_pretty().as_bytes());
}
