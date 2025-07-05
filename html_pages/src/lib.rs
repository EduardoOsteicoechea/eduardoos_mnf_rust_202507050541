// html_pages/src/lib.rs

// Declare the `page_template` module.
// `pub mod` makes the module public, so it can be accessed from other crates.
pub mod page_template;

// Declare the `home_page` module.
pub mod home_page;

// You can also re-export items for easier access from the consuming crate.
// For example, `pub use home_page::print_page;` would allow `use html_pages::print_page;`
// in `main.rs` instead of `use html_pages::home_page::print_page;`.
// For now, we'll keep it explicit for learning.