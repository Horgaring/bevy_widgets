use bevy::prelude::*;
use bevy_widgets::{widgets::progressbar::horizontal_bar::{ProgressBar, ProgressBarFill, ProgressSpeed}, UiWidgetPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiWidgetPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1))) // темный фон
        .insert_resource(ProgressSpeed(20.0)) // начальная скорость заполнения
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Камера для UI
    commands.spawn(Camera2d);

    // Контейнер прогресс-бара (фон)
    commands
        .spawn((Node {

            width: Val::Px(400.0),
            height: Val::Px(50.0),
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(100.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
            },
            BackgroundColor(Color::linear_rgb(0., 0., 1.))
        ))
        .with_children(|parent| {
            // Заполняемая часть прогресс-бара
            parent.spawn((
                Node {
                    width: Val::Percent(0.0), // начинаем с 0%
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::rgb(0.3, 0.7, 0.3)),
                ProgressBarFill,
            ));
        });

    

    // Компонент для хранения данных о прогрессе
    commands.spawn(ProgressBar {
        current: 0.0,
        max: 100.0,
    });
}