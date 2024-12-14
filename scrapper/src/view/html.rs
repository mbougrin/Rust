use crate::view::body::body;
use crate::view::head::head;

pub fn html_file() -> String {
    let doctype = String::from("<!DOCTYPE html>");
    let html = String::from("<html lang=\"en\">");
    let head = head();
    let body = body("");
    let html_end = String::from("</html>");
    let debug = format!("{doctype}{html}{head}{body}{html_end}");
    println!("{debug}");
    debug
}

pub fn html_file_search(search: &str) -> String {
    let doctype = String::from("<!DOCTYPE html>");
    let html = String::from("<html lang=\"en\">");
    let head = head();
    let body = body(search);
    let html_end = String::from("</html>");
    let debug = format!("{doctype}{html}{head}{body}{html_end}");
    println!("{debug}");
    debug
}