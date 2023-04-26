/*
 * These are returned by Component.update() and 
 * give information about what the window should do
 * afterwards
 */

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UpdateCommand{
    Void, // don't do anything
}