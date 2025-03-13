use bevy::{
    app::{ App, Startup },
    color::Color,
    core_pipeline::core_2d::Camera2d,
    ecs::system::Commands,
    hierarchy::{ BuildChildren, ChildBuild },
    text::{ TextColor, TextFont },
    ui::*,
    utils::default,
    DefaultPlugins,
};
use bevy_widgets::{ widgets::toggle::{ Toggle, ToggleState, ToggleText }, UiWidgetPlugin };

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiWidgetPlugin)
        .add_systems(Startup, setup)
        .run();
}
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Toggle {
                state: ToggleState::Off,
            },
            Node {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextFont {
                    font_size: 30.0,
                    ..Default::default()
                },
                TextColor(Color::linear_rgb(0.0, 0.0, 0.0)),
                ToggleText::default(),
            ));
        });
}
