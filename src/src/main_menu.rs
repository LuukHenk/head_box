// https://bevyengine.org/examples/Games/game-menu/

use bevy::{
    prelude::*,
    app::AppExit,
};
use super::GameState;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), main_menu_setup)
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::MainMenu)),
            );
    }
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct SelectedOption;

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
        background_color: NORMAL_BUTTON.into(),
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

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {game_state.set(GameState::Game);}
            }
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}