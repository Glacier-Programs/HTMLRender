use crate::update_commands::UpdateCommand;

type Component = Box<dyn ComponentObject>;

// This is to implemented on any struct
// which can be treated as a Component
pub trait ComponentObject{
    /*
     * on_init() 
     *     Called during Component instantiation
     * update()
     *     Called each frame
     * pre_render()
     *     Called between each screen render
     * get_sprite()
     *     Returns information needed for rendering
     */

    fn on_init(&mut self){
    
    }

    fn update(&mut self) -> UpdateCommand{
        UpdateCommand::Void
    }

    fn pre_render(&mut self){

    }

    fn get_sprite(&mut self){

    }
}