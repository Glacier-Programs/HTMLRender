/*
 * Components:
 *     - Rectangle: A rectangle on screen
 *     - TextBox: Just text with a background
 *     - Button: Can be clicked
 *     - PlugIn: Imported component
 */

pub enum Component{
    Rectangle,
    TextBox,
    Button,
    PlugIn
}

impl Component{
    /*
     * on_init() 
     *     Called during Component instantiation
     * update()
     *     Called each frame
     * pre_render()
     *     Called between each screen render
     */

    pub fn pre_render(){
    }
}