pub fn xcss(xpath: &str) -> String {
    let mut css = xpath
        // Remove the leading "//*" and any other leading slashes
        .trim_start_matches("//*").trim_start_matches('/')
        // Replace attribute selectors with equivalent CSS attribute selectors
        .replace("[@id='", "#").replace("']", "")
        // Replace numeric predicate expressions with ":nth-of-type"
        .replace("[", ":nth-of-type(").replace("]", ")");

    // Replace forward slashes with " > " and add the "#" prefix to the first selector
    if css.starts_with('#') {
        css = css.replace("/", " > ");
    } else {
        css = css
            .splitn(2, '/')
            .map(|part| format!("#{} > {}", part, part.replace("/", " > ")))
            .collect::<Vec<String>>()
            .join(" > ");
    }

    css
}
