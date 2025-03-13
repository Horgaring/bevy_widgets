use bevy::{app::{App, Startup}, color::Color, core_pipeline::core_2d::Camera2d, ecs::system::Commands, text::{TextColor, TextFont}, ui::{widget::Text, BackgroundColor, Node, UiRect, Val}, utils::default, DefaultPlugins};
use bevy_widgets::{widgets::tooltip::{TooltipBuilder, TooltipTrigger}, UiWidgetPlugin};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiWidgetPlugin)
        .add_systems(Startup, setup)
        .run();
}
pub fn setup(mut commands: Commands) {
    
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
        ..default()
    })
    .run_commands(|mut commands|{
        commands.insert(TextFont {
            font_size: 20.0,
            ..Default::default()
        });
        commands.insert(TextColor(Color::linear_rgb(0.0, 0.0, 0.0)));
        commands.insert(BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)));
        commands
    })
    .spawn(&mut commands);
    
}
