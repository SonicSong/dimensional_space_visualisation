use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use glow::*;
use std::rc::Rc;

pub fn object_sphere(x: f32, y: f32, w: f32, h: f32) {
    // std::vector<float> vertices;
    // float n1[3];        // normal of longitudinal plane rotating along Y-axis
    // float n2[3];        // normal of latitudinal plane rotating along Z-axis
    // float v[3];         // direction vector intersecting 2 planes, n1 x n2
    // float a1;           // longitudinal angle along Y-axis
    // float a2;           // latitudinal angle along Z-axis

    let vertices: [f32; 16];
    let n1: [f32; 3];
    let n2: [f32; 3];
    let v: [f32; 3];
    let a1: f32;
    let a2: f32;
}