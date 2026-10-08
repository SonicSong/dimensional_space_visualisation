use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use glow::*;
use std::rc::Rc;

// Part of the code responsbile for main rendering of stuff
pub fn sdl_visual() {
    // Init SDL2
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    // Request modern core OpenGL
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    // Create window + GL context
    let window = video
        .window("SDL2 + Glow Triangle", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();

    // Load OpenGL through glow
    let gl = Rc::new(unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    });

    // ----- SHADERS -----
    let vertex_shader_src = r#"
        #version 330 core
        layout (location = 0) in vec2 aPos;
        layout (location = 1) in vec3 aColor;
        out vec3 vColor;
        void main() {
            vColor = aColor;
            gl_Position = vec4(aPos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330 core
        in vec3 vColor;
        out vec4 FragColor;
        void main() {
            FragColor = vec4(vColor, 1.0);
        }
    "#;

    unsafe {
        let program = gl.create_program().expect("Cannot create program");

        let vs = gl.create_shader(VERTEX_SHADER).unwrap();
        gl.shader_source(vs, vertex_shader_src);
        gl.compile_shader(vs);
        assert!(gl.get_shader_compile_status(vs));

        let fs = gl.create_shader(FRAGMENT_SHADER).unwrap();
        gl.shader_source(fs, fragment_shader_src);
        gl.compile_shader(fs);
        assert!(gl.get_shader_compile_status(fs));

        gl.attach_shader(program, vs);
        gl.attach_shader(program, fs);
        gl.link_program(program);
        assert!(gl.get_program_link_status(program));

        gl.delete_shader(vs);
        gl.delete_shader(fs);

        gl.use_program(Some(program));

        // Triangle vertices
        let vertices: [f32; 15] = [
            // pos      // color
            -0.5, -0.5, 1.0, 0.0, 0.0,
            0.5, -0.5, 0.0, 1.0, 0.0,
            0.0,  0.5, 0.0, 0.0, 1.0,
        ];

        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();

        gl.bind_vertex_array(Some(vao));

        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices),
            STATIC_DRAW,
        );

        // position
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, 5 * 4, 0);

        // color
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 3, FLOAT, false, 5 * 4, 2 * 4);

        // Main loop
        let mut event_pump = sdl.event_pump().unwrap();
        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                    _ => {}
                }
            }

            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(COLOR_BUFFER_BIT);

            gl.draw_arrays(TRIANGLES, 0, 3);

            window.gl_swap_window();
        }

        gl.delete_program(program);
    }
}