
fn main() {
    let xpath = "//*[@id='foo']";
    let css_selector = xcss::xcss(xpath);
    println!("{}", css_selector);
}
