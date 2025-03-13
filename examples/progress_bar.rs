use bevy::prelude::*;
use bevy_widgets::widgets::progressbar::horizontal_bar::{
    ProgressBar,
    ProgressBarFill,
    ProgressBarWidgetPlugin,
    ProgressSpeed,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ProgressBarWidgetPlugin)
        .insert_resource(ClearColor(Color::linear_rgb(0.1, 0.1, 0.1))) // темный фон
        .insert_resource(ProgressSpeed(20.0)) // начальная скорость заполнения
        .add_systems(Startup, setup)
        .add_systems(Update, update_progress)
        .run();
}

fn setup(mut commands: Commands) {
    // Камера для UI
    commands.spawn(Camera2d);
    let mut progressbar = ProgressBar::new(100.0);
    progressbar.step = 10.0;
    // Контейнер прогресс-бара (фон)
    commands
        .spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(100.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderColor(Color::WHITE),
            progressbar,
        ))
        .with_children(|parent| {
            // Заполняемая часть прогресс-бара
            parent.spawn((
                Node {
                    width: Val::Percent(0.0), // начинаем с 0%
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::Srgba(Srgba::hex("#838383").unwrap())),
                ProgressBarFill,
            ));
        });
}
fn update_progress(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut ProgressBar>,
    mut active: Local<bool>
) {
    if keys.pressed(KeyCode::KeyA) 
        && !*active {
        *active = true;
    }
    if *active {
        let mut progress = q.get_single_mut().unwrap();
        progress.current += progress.step * time.delta_secs();
        if progress.current >= progress.get_max() {
            *active = false;
            progress.current = 0.0;
        }
        
    }
}
