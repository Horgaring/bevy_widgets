use bevy::{
    app::{ App, Plugin, Update }, ecs::{ component::Component, query::{ Changed, Or }, system::Query }, hierarchy::Children, ui::{ widget::{ Button, Text }, Interaction }
};

#[derive(Debug, Clone, Component)]
#[require(Button)]
pub struct Toggle {
    pub state: ToggleState,
}

pub struct ToggleWidgetPlugin;

impl Plugin for ToggleWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_system, toggle_text));
    }
}

#[derive(Component)]
#[require(Text)]
pub struct ToggleText {
    pub on_text: String,
    pub off_text: String,
}
impl Default for ToggleText {
    fn default() -> Self {
        ToggleText {
            on_text: "On".to_string(),
            off_text: "Off".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ToggleState {
    On,
    Off,
}

pub fn toggle_system(
    mut query: Query<(&mut Toggle, &Interaction), Or<(Changed<Toggle>, Changed<Interaction>)>>
) {
    for (mut toggle, interaction) in query.iter_mut() {
        if interaction == &Interaction::Pressed {
            toggle.state = match toggle.state {
                ToggleState::On => ToggleState::Off,
                ToggleState::Off => ToggleState::On,
            };
        }
    }
}
pub fn toggle_text(
    mut toggle_query: Query<(&mut Toggle, &Children)>,
    mut toogle_text_query: Query<(&mut ToggleText, &mut Text)>
) {
    for (mut toggle, mut child) in toggle_query.iter_mut() {
        for (mut toggle_text, mut text) in toogle_text_query.get_mut(child[0]) {
            match toggle.state {
                ToggleState::On => {
                    text.0 = toggle_text.on_text.clone();
                }
                ToggleState::Off => {
                    text.0 = toggle_text.off_text.clone();
                }
            }
        }
    }
}
