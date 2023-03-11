# Xcss

Rust Package to Convert the Xpath selectors to CSS selector.


## Usage

```rust
fn main() {
    let xpath = "//*[@id='foo']";
    let css_selector = xcss::xcss(xpath);
    println!("{}", css_selector);
}

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)