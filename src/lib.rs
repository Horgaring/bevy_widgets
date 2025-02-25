use bevy::{app::{App, Plugin, PostUpdate, Startup, Update}, ui::update};
use widgets::{slider::{slide, SliderResource}, toggle::{toggle_system, toggle_text}, tooltip::show_tooltip};

pub mod widgets;

pub struct UiWidgetPlugin;

impl Plugin for UiWidgetPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SliderResource{active: None})
            .add_systems(Update, (toggle_system, toggle_text, show_tooltip))
            .add_systems(PostUpdate, slide);
    }
}