# Xcss

🦀Rust Package to Convert the Xpath selectors to CSS selector.


## Usage

```rust
use xcss::xcss;

fn main() {
    let xpath = "/html/body/div[2]/section[1]";
    let css = xcss::xcss(xpath);
    println!("{}", css);
}

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
