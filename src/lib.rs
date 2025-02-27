mod shared;
pub use shared::despawn_systems::despawn_screen;
pub use shared::game_state::GameState;

mod main_menu;
pub use main_menu::plugin::MainMenuPlugin;

mod game;
pub use game::plugin::GamePlugin;
