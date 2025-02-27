use bevy::prelude::*;

use crate::{despawn_entities, GameState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup);
        app.add_systems(
            OnExit(GameState::Menu),
            despawn_entities::<OnMainMenuScreen>,
        );
        app.add_systems(
            Update,
            (button_system, menu_action).run_if(in_state(GameState::Menu)),
        );
    }
}

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.3, 0.3, 0.3);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => NORMAL_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => NORMAL_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Camera2d::default(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|button_area_parent| {
            spawn_menu_button(button_area_parent, "Play", MenuButtonAction::Play);
            spawn_menu_button(button_area_parent, "Quit", MenuButtonAction::Quit);
        });
}

fn spawn_menu_button(parent: &mut ChildBuilder, text: &str, button_action: MenuButtonAction) {
    parent
        .spawn(create_button_bundle(button_action))
        .with_children(|button_parent| {
            button_parent.spawn(create_text_bundle(text));
        });
}

fn create_text_bundle(text: &str) -> (Text, TextFont) {
    (
        Text(text.to_string()),
        TextFont {
            font_size: 40.0,
            ..default()
        },
    )
}

fn create_button_bundle(
    button_action: MenuButtonAction,
) -> (Button, MenuButtonAction, BackgroundColor, Node) {
    (
        Button,
        button_action,
        BackgroundColor(NORMAL_BUTTON),
        Node {
            width: Val::Px(300.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )
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
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                }
            }
        }
    }
}
