use crate::scrap_source::scrap::search_network;

fn html_form_search(search: &str) -> String {
    let div = String::from("<div class=\"form_class\">");
    let form = String::from("<form method=\"get\" id=\"form_search\">");
    let input = String::from("<input type=\"text\" id=\"search\" name=\"search\"");
    let value_search = format!(" value=\"{search}\">");
    let button = String::from("<button type=\"button\" value=\" Click\" onclick=\"send()\">Search</button>");
    let form_end = String::from("</form>");
    let div_end = String::from("</div>");
    format!("{div}{form}{input}{value_search}{form_end}{button}{div_end}")
}

fn adding_source() -> String {
    let begin_form = String::from("<form method=\"get\" id=\"form_source\" action=\"/source\">");
    let input_google = String::from("<label for=\"google_search\">Google Search</label><input style=\"float: right;\" type=\"checkbox\" id=\"google_search\" name=\"Google Search\" unchecked onclick=\"sendSource()\"/><hr>");
    let end_form = String::from("</form");
    format!("{begin_form}{input_google}{end_form}")
}
fn sidebar_div_left() -> String {
    let navbar = String::from("<navbar class=\"sidebar\">");
    let navbarscroll = String::from("<div class=\"sidebar-scrollbox\">");
    let source = adding_source();
    let navbarend = String::from("</div></div>");
    format!("{navbar}{navbarscroll}{source}{navbarend}")
}

fn div_result() -> String  {
    let begin_div = String::from("<div class=\"search\">");
    //todo add information div
    let end_div = String::from("</div>");
    format!("{begin_div}{end_div}")
}

pub fn body(search: &str) -> String {
    let body = String::from("<body>");
    let form_search = html_form_search(search);
    if search != "" {
        search_network(search);
    }
    let sidebar_left = sidebar_div_left();
    let search_div = div_result();
    let body_end = String::from("</body");
    format!("{body}{form_search}{sidebar_left}{search_div}{body_end}")
}