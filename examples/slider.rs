use bevy::{
    app::{ App, Startup },
    color::Color,
    core_pipeline::core_2d::Camera2d,
    ecs::system::Commands,
    ui::BackgroundColor,
    DefaultPlugins,
};
use bevy_widgets::{
    widgets::slider::SliderBuilder,
    UiWidgetPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiWidgetPlugin)
        .add_systems(Startup, setup)
        .run();
}
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let entities = SliderBuilder::new().spawn(&mut commands);
    commands.entity(entities.dot).insert(BackgroundColor(Color::linear_rgb(0.4, 0.4, 0.4)));
    commands.entity(entities.line).insert(BackgroundColor(Color::linear_rgb(0.0, 0.0, 0.0)));
}
