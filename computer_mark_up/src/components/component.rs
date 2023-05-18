use crate::{
    update_commands::UpdateCommand, 
    render::{vertex::ComponentVertex, texture::Texture}, input_handler::InputHandler
};

// Box<> is a smart pointer which allows the reference to be borrowed
// It also makes it much easier to manage storing unknown sized data than
// using raw pointers or borrows
pub type Component = Box<dyn ComponentObject>;

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

    fn on_init(&mut self){}

    fn update(&mut self, input: &InputHandler) -> UpdateCommand{ UpdateCommand::Void }

    fn pre_render(&mut self) -> &Texture;

    fn get_vertices(&self) -> [ComponentVertex; 4];
}

// A component can have three build options:
// -> Default Build
// -> Structured Build
// -> Both of them
// A default build will always make the same component
// with the same specifications
// A structured build will take a custom parameters object 
// and will use it to construct the object
//
// Either way, the component needs to implement
// the build process on its own
//
// Builders are used to make it easier to construct
// components at runtime

pub trait DefaultBuild: ComponentObject{
    fn build_default(device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) -> Self;
}

pub trait CustomBuildParameters{}
pub trait CustomBuild: ComponentObject{
    fn build_custom<P: CustomBuildParameters>(device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration, parameters: P) -> Self;
}