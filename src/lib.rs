//General structs and methods.
pub mod core;

//Structs built off of core specific to ray tracing.
pub mod ray_tracing;

//Object properties such as refraction and color
pub mod materials;

//Structs which render the scene and store scene objects
pub mod world;

//Objects rendered with ray tracing
pub mod objects;

//Miscellaneous methods and structs
pub mod misc;

//Various exercises testing the functionality of core structs
mod exercises;
