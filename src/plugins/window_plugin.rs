.add_plugins(DefaultPlugins.set(WindowPlugin {
    primary_window: Some(Window {
        title: "HeadBox".into(),
        resolution: (1920., 1080.).into(),
        ..default()
    }),
    ..default()
}))