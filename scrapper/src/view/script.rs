fn send() -> String {
    String::from("\
    function send() {\
    document.getElementById(\"form_search\").submit();\
    }")
}

fn send_source() -> String {
    String::from("\
    function sendSource() {\
    document.getElementById(\"form_source\").submit();\
    }")
}
pub fn script() -> String {
    let begin = String::from("<script type=\"text/javascript\">");
    let send = send();
    let send_source = send_source();
    let end = String::from("</script>");
    format!("{begin}{send}{send_source}{end}")
}