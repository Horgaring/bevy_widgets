use std::process::Command;

use bevy::{color::Color, ecs::{component::Component, query::{Changed, Or}, system::{Commands, Query}}, hierarchy::{BuildChildren, ChildBuild}, text::{JustifyText, Text2d, TextColor, TextLayout}, ui::{widget::{Button, Text}, Interaction, Node}};

#[derive(Debug, Clone, Component)]
#[require(
    Button,
)]
pub struct Toggle {
    pub state: ToggleState,
}

#[derive(Component)]
#[require(
    Text,
)]
pub struct ToggleText
{
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
    mut query: Query<
        (&mut Toggle, &Interaction),
        Or<(Changed<Toggle>, Changed<Interaction>)>,
    >,
) {
    for (mut toggle,  interaction) in query.iter_mut() {
        if interaction == &Interaction::Pressed {
            toggle.state = match toggle.state {
                ToggleState::On => ToggleState::Off,
                ToggleState::Off => ToggleState::On,
            };
        }
    }
}
pub fn toggle_text(
    mut toggle_query: Query<(&mut Toggle,&mut Text,&mut ToggleText)>,
) {
    for (mut toggle,mut text,mut toggle_text) in toggle_query.iter_mut() {
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


