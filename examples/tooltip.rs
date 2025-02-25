//! Illustrates bloom post-processing in 2d.

use std::thread::spawn;

use bevy::{app::{App, Startup, Update}, color::Color, core_pipeline::core_2d::{Camera2d, Camera2dBundle}, ecs::system::{Commands, Query}, math::Vec2, sprite::{Sprite, SpriteBundle}, text::{self, Text2d, TextColor, TextFont}, transform::components::Transform, ui::{node_bundles::ButtonBundle, widget::{Button, Text}, BackgroundColor, Interaction, Node, UiRect, Val}, utils::default, DefaultPlugins};
use bevy_widgets::{widgets::{toggle::{Toggle, ToggleState, ToggleText}, tooltip::{TooltipBuilder, TooltipTrigger}}, UiWidgetPlugin};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiWidgetPlugin)
        .add_systems(Startup, setup)
        .run();
}
pub fn setup<'w, 's>(mut commands: Commands<'w, 's>) {
    
    commands.spawn(Camera2d);
    commands.spawn((
        Text("Hello".to_string()),
        TooltipTrigger{
            content: "Hello".to_string()
        },
    ));
    commands.spawn((
        Text("Hello".to_string()),
        TooltipTrigger{
            content: "Hello".to_string()
        },
        Node{
            margin: UiRect::all(Val::Auto),
            ..default()
        }
    ));
    commands.spawn((
        Text("Hello".to_string()),
        TooltipTrigger{
            content: "Hello".to_string()
        },
        Node{
            margin: UiRect::new(Val::Auto, Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
            ..default()
        }
    ));
    commands.spawn((
        Text("Hello".to_string()),
        TooltipTrigger{
            content: "Hello".to_string()
        },
        Node{
            margin: UiRect::new(Val::Px(0.0), Val::Auto, Val::Auto, Val::Px(0.0)),
            ..default()
        }
    ));
    commands.spawn((
        Text("Hello".to_string()),
        TooltipTrigger{
            content: "Hello".to_string()
        },
        Node{
            margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Auto, Val::Px(0.0)),
            ..default()
        }
    ));
    TooltipBuilder::new()
    .with_content(Node{
        margin: UiRect::all(Val::Px(10.)),
        ..default()
    })
    .run_commands(|mut commands|{
        commands.insert(TextFont {
            font_size: 20.0,
            ..Default::default()
        });
        commands.insert(TextColor(Color::linear_rgb(0.0, 0.0, 0.0)));
        commands.insert(BackgroundColor(Color::rgb(1.0, 0.0, 0.0)));
        commands
    })
    .spawn(&mut commands);
    
}
