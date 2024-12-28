#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub mod desktop;

#[cfg(not(target_arch = "wasm32"))]
pub mod tui;
