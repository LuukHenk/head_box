use bevy::{
    prelude::*,
};
use super::GameState;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const DEFAULT_MENU_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(OnEnter(GameState::MainMenu), main_menu_setup);
    }
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}




fn get_default_menu_text_style() -> TextStyle {
    TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    }
}

fn get_default_menu_text_bundle(text: &str) -> TextBundle {
    TextBundle::from_section(
        text,
        get_default_menu_text_style(),
    )
}
fn get_default_menu_button_style() -> Style {
    Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
fn get_default_menu_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: get_default_menu_button_style(),
        background_color: DEFAULT_MENU_BUTTON_COLOR.into(),
        ..default()
    }
}

fn spawn_main_menu_button(parent: &mut ChildBuilder, text: &str, action: MenuButtonAction) {
    parent.spawn((
        get_default_menu_button_bundle(),
        action,
    )).with_children(|parent| {
        parent.spawn(get_default_menu_text_bundle(text));
    });
}

fn create_main_menu_layout() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::CRIMSON.into(),
        ..default()
    }
}

fn main_menu_setup(mut commands: Commands) {
    commands.spawn(create_main_menu_layout()).with_children(|parent| {
        spawn_main_menu_button(parent, "Start", MenuButtonAction::Play);
        spawn_main_menu_button(parent, "Quit", MenuButtonAction::Quit);
    });
}