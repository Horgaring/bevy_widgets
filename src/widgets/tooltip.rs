use bevy::{
    ecs::{component::Component, entity::Entity, query::With, system::{Commands, EntityCommands, Local, Query, Res}}, render::view::{window, Visibility}, scene::ron::de::Position, ui::{
        widget::{Button, Text}, ComputedNode, Interaction, Node, PositionType, RelativeCursorPosition, Val
    }, utils::{default, info}, window::Window
};

pub struct TooltipBuilder<'a> {
    content: Option<Node>,
    commands: Vec<Box<dyn for<'b> Fn(EntityCommands) -> EntityCommands + 'a>>,
}
#[derive(Component)]
pub struct Tooltip;
#[derive(Component)]
#[require(Button, RelativeCursorPosition)]
pub struct TooltipTrigger {
    pub content: String,
}
impl<'a> TooltipBuilder<'a> {
    pub fn new() -> Self {
        TooltipBuilder {
            content: None,
            commands: Vec::new(),
        }
    }
    pub fn with_content(&mut self,mut  content: Node) -> &mut Self {
        content.position_type = PositionType::Absolute;
        self.content = Some(content);
        self
    }
    pub fn run_commands(&mut self, extend: impl Fn(EntityCommands) -> EntityCommands + 'a) -> &mut Self {
        self.commands.push(Box::new(extend));
        self
    }
    pub fn spawn(&mut self, commands: &mut Commands) -> Entity  {
        let mut entcom = commands
            .spawn((
                Tooltip, 
                Visibility::Hidden, 
                Text("".to_string()),
                self.content.take().unwrap(),
            ));
            let mut entcom2 = entcom;
        for command in self.commands.iter() {
            entcom2 = command(entcom2);
            
        }
        entcom2.id() 
    }
}

impl<'a> Default for TooltipBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn show_tooltip(
    mut showed_id: Local<Option<Entity>>,
    mut query : Query<(&mut Visibility, &mut Text,&mut Node,&ComputedNode),With<Tooltip>>,
    trigger_query: Query<(Entity,&TooltipTrigger, &Interaction)>,
    windows: Query<&Window>,
){
    if let Some(cursor_position) = windows.single().cursor_position(){
        for (entity,trigger, interaction) in trigger_query.iter() {
            if let Some(id) = *showed_id {
                if interaction != &Interaction::Hovered
                    && entity == id{
                    *showed_id = None;
                    if let Ok((mut visibility, _,_,_)) = query.get_single_mut(){
                        *visibility = Visibility::Hidden;
                    }
                    
                }
            }
            if interaction == &Interaction::Hovered {
                *showed_id = Some(entity);
                if let Ok((mut visibility, mut text,mut node,size)) = query.get_single_mut()
                {
                    *visibility = Visibility::Visible;
                    text.0 = trigger.content.clone();
                    if windows.single().size().y < cursor_position.y + size.size().y {
                        node.top = Val::Px(cursor_position.y - size.size().y );
                    }
                    else {
                    
                        node.top = Val::Px(cursor_position.y + 20.);
                    }
                    let pos = cursor_position.x - size.size().x / 2.;
                    if windows.single().size().x < pos + size.size().x {
                        node.left = Val::Px(windows.single().size().x - size.size().x);
                    }
                    else if pos < 0. {
                        node.left = Val::Px(0.);
                    }
                    else {
                        node.left = Val::Px(pos);
                    }
                }
            }
            
    
        }
    }
}