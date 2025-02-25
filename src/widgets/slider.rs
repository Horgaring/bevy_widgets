use bevy::{color::Color, ecs::{component::Component, entity::Entity, query::{With, Without}, system::{Commands, Query, Res, ResMut, Resource}}, hierarchy::{BuildChildren, ChildBuild, Children, Parent}, input::{mouse::MouseButton, ButtonInput}, log::info, transform::components::GlobalTransform, ui::{widget::Button, BackgroundColor, BorderRadius, Interaction, Node, PositionType, RelativeCursorPosition, UiRect, Val}, utils::{default, info}, window::Window};

pub struct SliderBuilder{
    pub line: Option<Node>,
    pub dot: Option<Node>
}
impl Default for SliderBuilder{
    fn default() -> Self {
        Self::new()
    }
}
pub struct SliderEntities{
    pub line: Entity,
    pub dot: Entity
}
impl SliderBuilder{
    pub fn new() -> Self{
        let lineheight = 6.0;
        let dotr = 20.0;
        Self{
            line: Some(Node{
                width: Val::Px(100.0),
                height: Val::Px(lineheight),
                position_type: PositionType::Relative,
                margin: UiRect::all(Val::Auto),
                ..default()
            }),
            dot: Some(Node{
                position_type: PositionType::Absolute,
                top: Val::Px(-(dotr - lineheight) / 2.0),
                width: Val::Px(dotr),
                height: Val::Px(dotr),
                ..default()
            })
        }
    }
    pub fn spawn(&mut self,commands: &mut Commands,) -> SliderEntities{
        let line_id = commands.spawn((
            self.line.take().unwrap(),
            SliderLine,
            Button,
            RelativeCursorPosition::default(),
        )).id();
        let mut dot_id = Entity::from_raw(0);
        commands.entity(line_id).with_children(|parent| {
            dot_id = parent.spawn((
                self.dot.take().unwrap(),
                BorderRadius::all(Val::Percent(100.0)),
                SliderDot,
            )).id();
        });
        SliderEntities{
            line: line_id,
            dot: dot_id
        }
    }
}
//
#[derive(Component)]
pub struct SliderDot;
#[derive(Component)]
pub struct SliderLine;
#[derive(Resource)]
pub struct SliderResource{
    pub active: Option<Entity>,
}
pub fn slide(
    mut query: Query<(&mut RelativeCursorPosition,&mut Node,&Interaction,&mut Children),With<SliderLine>>,
    mut dot_query: Query<(&mut Node),Without<SliderLine>>,
    mut slider_res: ResMut<SliderResource>,
){
    for (cursor_pos,node,interaction,children) in query.iter_mut(){
        if interaction == &Interaction::Pressed{
            if let Some(pos) = cursor_pos.normalized{
                
                let a = children.iter().next().unwrap();
                let mut dot = dot_query.get_mut(*a).unwrap();
                let width = match &node.width{
                    Val::Px(v) => *v,
                    _ => 0.0
                    
                };
                let dot_width = match &dot.width{
                    Val::Px(v) => *v,
                    _ => 0.0
                };
                dot.left = Val::Px((pos.x * width - dot_width / 2.0)
                    .clamp(-dot_width / 2., width - dot_width / 2.));
            }
        }
    }
}