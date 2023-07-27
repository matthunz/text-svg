![example](https://raw.githubusercontent.com/matthunz/text-svg/main/image.svg)

Text -> SVG path in rust

[![Latest Version](https://img.shields.io/crates/v/text-svg.svg)](https://crates.io/crates/text-svg)
[![documentation](https://docs.rs/text-svg/badge.svg)](https://docs.rs/text-svg)
![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)

[Examples](https://github.com/matthunz/text-svg/tree/main/examples)

```rust
Text::builder()
    .size(50.0)
    .start(Point { x, y })
    .build(&font, "text-svg");
```
