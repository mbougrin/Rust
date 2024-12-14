const SIDE_BAR_WIDTH: &str = "150px";
const BACKGROUND_COLOR: &str = "#f1f6f9";
fn sidebar_left() -> String {
    String::from("\
    .sidebar {\
    position: fixed;\
    left: 0;\
    top: 45px;\
    bottom: 0;\
    width: %1;\
    font-size: 0.875em;\
    box-sizing: border-box;\
    -webkit-overflow-scrolling: touch;\
    overscroll-behavior-y: contain;\
    background-color: %2;\
    color: #131313;\
    border-right:solid;\
    border-color:darkgrey;\
    }").replace("%1", SIDE_BAR_WIDTH)
        .replace("%2", BACKGROUND_COLOR)
}

fn navbar_scroll() -> String {
    String::from("\
    .sidebar .sidebar-scrollbox {\
    overflow-y: auto;\
    position: absolute;\
    top: 0;\
    bottom: 0;\
    right: 0;\
    }")
}

fn form_source_design() -> String {
    String::from("\
    .form_search {\
    width:%1;\
    }").replace("%1", SIDE_BAR_WIDTH)
}

fn form_search() -> String {
    let class_form = String::from("\
    .form_class {\
    height:42px;\
    text-align:center;\
    border-bottom:solid;\
    border-color:darkgrey;\
    background-color: %1;\
    }").replace("%1", BACKGROUND_COLOR);
    let form = String::from("\
    .form_class form {\
    margin-top:10px;\
    display:inline-block;\
    }");
    format!("{class_form}{form}")
}

fn hr() -> String {
    String::from("\
    hr{\
    border:1px solid darkgrey;\
    }")
}

fn body() -> String {
    String::from("\
    body{\
    margin:0;\
    width:100%;\
    }")
}

pub fn css_file() -> String {
    let style = String::from("<style>");
    let body_design = body();
    let hr = hr();
    let form_design = form_search();
    let sidebar_left = sidebar_left();
    let scroll = navbar_scroll();
    let form_source = form_source_design();
    let style_end = String::from("</style>");
    format!("{style}{body_design}{hr}{form_design}{sidebar_left}{form_source}{style_end}")
}