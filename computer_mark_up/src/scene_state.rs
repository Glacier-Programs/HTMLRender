use crate::components::{Component, ComponentObject};

pub struct SceneState{
    components: Vec<Component>
}

impl SceneState{
    pub fn new_empty() -> Self {
        Self { 
            components: Vec::new()
         }
    }

    // this may have to be changed to 'a eventually
    pub fn add_component<C: ComponentObject + 'static>(&mut self, component: C){
        self.components.push( Box::new(component) )
    }

    pub fn get_components(&self) -> &[Component]{
        self.components.as_slice()
    }
}