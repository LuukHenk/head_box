#![feature(const_option)]

mod assets;
mod display_handler;
mod events;
mod utils;
mod components;
mod systems;
mod states;
mod plugins;

pub use plugins::head_box_plugin::HeadBoxPlugin;

// println!("{:#?}", something);
