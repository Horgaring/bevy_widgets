use bevy::app::{ App, Plugin };
use widgets::{
    slider::SliderWidgetPlugin,
    toggle::ToggleWidgetPlugin,
    tooltip::TooltipWidgetPlugin,
};

pub mod widgets;

pub struct UiWidgetPlugin;

impl Plugin for UiWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ToggleWidgetPlugin)
            .add_plugins(SliderWidgetPlugin)
            .add_plugins(TooltipWidgetPlugin);
    }
}
