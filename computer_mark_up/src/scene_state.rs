use crate::{
    components::{
        Component, 
        ComponentObject,
        DefaultBuild,
        CustomBuild,
        CustomBuildParameters
    }, 
    input_handler::InputHandler, render::window_state::WindowState
};

pub struct SceneState{
    components: Vec<Component>
}

impl SceneState{
    pub fn new_empty() -> Self {
        Self { 
            components: Vec::new()
         }
    }

    /*
     * IMPORTANT NOTE:
     *    Every component object added to the scene has a static lifetime
     *    This means that the data will never be dropped. Components should 
     *    therefore be altered instead of replaced as replacing would lead to 
     *    a memory leak
     */

    // this may have to be changed to 'a eventually
    pub fn add_component<C: ComponentObject + 'static>(&mut self, component: C){
        self.components.push( Box::new(component) )
    }

    pub fn build_default_component<C: DefaultBuild + 'static>(&mut self, ws: &WindowState){
        self.components.push(
            Box::new(
                C::build_default(ws.device(), ws.queue(), ws.config())
            )
        )
    }

    pub fn build_custom_component<C: CustomBuild + 'static>(&mut self, ws: &WindowState, parameters: impl CustomBuildParameters){
        self.components.push(
            Box::new(
                C::build_custom(ws.device(), ws.queue(), ws.config(), parameters)
            )
        )
    }

    pub fn get_components(&self) -> &[Component]{
        self.components.as_slice()
    }

    // fun parts
    pub fn update(&mut self, input: &InputHandler){
        for comp in &mut self.components{
            let command = comp.update(input);
        }
    }
}