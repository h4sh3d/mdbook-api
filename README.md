[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **THIS PROJECT HAS BEEN ARCHIVED**

`mdbook-api`
===

`mdbook-api` is a `mdBook` backend that ports the work of [Slate](https://github.com/slatedocs/slate).

Assets in the `theme/` folder come from [Slate](https://github.com/slatedocs/slate) project.

## Usage

Install `mdbook-api` backend with

```
cargo install --git https://github.com/h4sh3d/mdbook-api.git
```

and configure your `book.toml`

```toml
[book]
# ...

[output.api]

# A list of links for TOC footer
[[output.api.toc_footer]]
link_url = "https://example.com"
content = "Get in touch"

# A list of languages for constructing the menu
[[output.api.lang]]
# Language id
id = "rust"
# Display name, optional
name = "Rust"

[[output.api.lang]]
id = "go"
name = "Go"
```

## Licence

All the code in this repository is released under the MIT, for more information take a look at the [LICENSE](LICENSE) file.
