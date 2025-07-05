pub mod components;

// Re-export specific components for easier access from consuming crates.
// This allows `use pages_components::ButtonComponent;` instead of `use pages_components::components::button_component::ButtonComponent;`
pub use components::button_component::ButtonComponent;