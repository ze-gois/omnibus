#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub use app::TemplateApp;

pub mod gui;

pub mod global;

pub mod types;

pub mod plugins;
