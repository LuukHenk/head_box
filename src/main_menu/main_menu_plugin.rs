use bevy::{
    app::{App, Plugin, Startup},
    ecs::system::Commands,
    hierarchy::{BuildChildren, ChildBuild},
    text::TextFont,
    ui::{
        widget::{Button, Text},
        AlignItems, FlexDirection, JustifyContent, Node,
    },
    utils::default,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main_menu);
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Button,
        ))
        .with_children(|parent| {
            (parent.spawn((
                Text("Hello".to_string()),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
            )));
        });
}
