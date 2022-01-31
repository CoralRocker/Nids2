/// Module to hold game-system functions and static variables
pub mod game;

/// Module to hold code for controlling and configuring the player character. 
pub mod naomi;

/// Module to define what objects must implement and provide a generic default object type which
/// can create any object defined in `crate::game`
pub mod object;

/// Module to hold utility code. Mostly drawing functions, but includes some functions to mess with
/// data, objects, and other useful utils. 
pub mod util;
