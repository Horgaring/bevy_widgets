use bevy::prelude::*;

pub struct ProgressBarWidgetPlugin;

impl Plugin for ProgressBarWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_progress_bar);
    }
}

#[derive(Component)]
pub struct ProgressBar {
    pub current: f32, 
    max: f32, 
    pub step: f32
}
impl ProgressBar{
    pub fn new(max: f32) -> Self{
        Self{
            current: 0.,
            max,
            step: 1.
        }
    }
    pub fn get_max(&self) -> f32{
        self.max
    }
}
impl Default for ProgressBar{
    fn default() -> Self {
        Self::new(100.)
    }
}
#[derive(Component)]
pub struct ProgressBarFill;

#[derive(Resource)]
pub struct ProgressSpeed(pub f32);

pub fn update_progress_bar(
    progress_query: Query<&ProgressBar>,
    mut fill_query: Query<&mut Node, With<ProgressBarFill>>
) {
    if let Ok(progress) = progress_query.get_single() {
        if let Ok(mut node) = fill_query.get_single_mut() {
            let fill_percent = (progress.current / progress.max) * 100.0;
            node.width = Val::Percent(fill_percent);
        }
    }
}
