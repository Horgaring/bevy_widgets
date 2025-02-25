//! Illustrates bloom post-processing in 2d.

use std::thread::spawn;

use bevy::{app::{App, Startup, Update}, color::Color, core_pipeline::core_2d::{Camera2d, Camera2dBundle}, ecs::system::{Commands, Query}, math::Vec2, sprite::{Sprite, SpriteBundle}, text::{Text2d, TextColor, TextFont}, transform::components::Transform, ui::{node_bundles::ButtonBundle, widget::Button, BackgroundColor, Interaction, Node, Val}, utils::default, DefaultPlugins};
use bevy_widgets::{widgets::{slider::SliderBuilder, toggle::{Toggle, ToggleState, ToggleText}}, UiWidgetPlugin};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiWidgetPlugin)
        .add_systems(Startup, setup)
        .run();
}
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let entities = SliderBuilder::new()
    .spawn(&mut commands);
    commands.entity(entities.dot).insert(
        BackgroundColor(Color::linear_rgb(0.4, 0.4, 0.4)),
    );
    commands.entity(entities.line).insert(
        BackgroundColor(Color::linear_rgb(0.0, 0.0, 0.0)),
    );
}
