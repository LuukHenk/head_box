#![feature(const_option)]

mod assets;
mod display_handler;
mod events;
pub mod generic_constants;
mod head_box_plugin;
mod in_game;
mod menus;
mod proof_of_concepts;
mod utils;

pub use head_box_plugin::head_box_plugin::HeadBoxPlugin;

// println!("{:#?}", something);
