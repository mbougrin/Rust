pub fn parse_search(line: &str) -> &str {
    let mut split = line.split(" ");
    let mut collection: Vec<&str> = split.collect();
    if collection.len() <= 1 {
        return "";
    }
    let index = collection.get(1).unwrap();
    if index.to_string() == "/"
        || index.to_string() == "/?search=" {
        return "";
    }
    if index.contains("search=") {
        split = index.split("search=");
        collection = split.collect();
        return collection.get(1).unwrap();
    }
    ""
}