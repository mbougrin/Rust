use crate::view::css::css_file;
use crate::view::meta::meta;
use crate::view::script::script;

pub fn head() -> String {
    let head = String::from("<head>");
    let style = css_file();
    let meta  = meta();
    let script = script();
    let head_end = String::from("</head>");
    format!("{head}{style}{meta}{script}{head_end}")
}