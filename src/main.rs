use raylib::prelude::*;
const INITIAL_WINDOW_WIDTH: i32 = 800;
const INITIAL_WINDOW_HEIGHT: i32 = 450;

fn main() {
    let (mut handle, thread) = raylib::init()
        .size(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
        .title("Window")
        .resizable()
        .vsync()
        .msaa_4x()
        .log_level(TraceLogLevel::LOG_NONE)
        .build();

    let mut virtual_width: u32 = 0;
    let mut virtual_height: u32 = 0;
    let monitor = get_current_monitor();

    virtual_width = get_monitor_width(monitor) as u32;
    virtual_height = get_monitor_height(monitor) as u32;

    let mut cam = Camera3D::perspective(rvec3(0, 1, 4), rvec3(0, 0, 0), rvec3(0, 1, 0), 45.0);

    match handle.load_render_texture(&thread, virtual_width, virtual_height) {
        Ok(mut target) => {
            target.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
            let mut shader = handle
                .load_shader(
                    &thread,
                    Some("./shader/simpleLight.vs"),
                    Some("./shader/simpleLight.fs"),
                )
                .unwrap();

            shader.locs_mut()[ShaderLocationIndex::SHADER_LOC_MATRIX_MODEL as usize] =
                shader.get_shader_location("matModel");
            shader.locs_mut()[ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize] =
                shader.get_shader_location("viewPos");

            let amb = shader.get_shader_location("ambient");
            shader.set_shader_value(amb, [0.2, 0.2, 0.2, 1.0]);
            let mut model = handle.load_model_from_mesh(&thread, unsafe{Mesh::gen_mesh_cube(&thread, 1., 1., 1.).make_weak()}).unwrap();

            //model.materials_mut()[0].shader = *shader;

            let mut angle = Vector3::zero();

            let mut fs_toggle = false;

            let mut dt = 0.0;

            while !handle.window_should_close() {
                handle.update_camera(&mut cam, CameraMode::CAMERA_FREE);
                dt = handle.get_frame_time();

                let screen_scale = (handle.get_screen_width() as f32 / virtual_width as f32)
                    .min(handle.get_screen_height() as f32 / virtual_height as f32);

                handle.set_mouse_scale(1.0 / screen_scale, 1.0 / screen_scale);
                handle.set_mouse_offset(rvec2(
                    -(handle.get_screen_width() as f32 - (virtual_width as f32 * screen_scale))
                        * 0.5,
                    -(handle.get_screen_height() as f32 - (virtual_height as f32 * screen_scale))
                        * 0.5,
                ));


                angle.x += 1.0 * dt;
                angle.y += 0.5 * dt;
                angle.z += 2.5 * dt;

                model.transform = Matrix::rotate_xyz(angle).into();

                shader.set_shader_value(
                    ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as i32,
                    cam.position.x,
                );

                let mut d = handle.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                {
                    let mut d = d.begin_texture_mode(&thread, &mut target);
                    d.clear_background(Color::LIGHTCYAN);
                    {
                        let mut d = d.begin_mode3D(cam);
                        d.draw_model(&model, rvec3(0, 0, 0), 1.0, Color::BLACK);
                        d.draw_grid(50, 0.5);
                    }
                    d.draw_fps(10, 10);
                    d.draw_text(
                        format!("screen scale {:?}", screen_scale).as_str(),
                        20,
                        80,
                        40,
                        Color::BLACK,
                    );

                    let mp = d.get_mouse_position();
                    d.draw_circle(mp.x as i32, mp.y as i32, 12.0, Color::RED);
                }

                d.draw_texture_pro(
                    target.texture(),
                    Rectangle::new(
                        0.0,
                        0.0,
                        target.texture.width as f32,
                        -target.texture.height as f32,
                    ),
                    Rectangle::new(
                        0.0,
                        0.0,
                        d.get_screen_width() as f32,
                        d.get_screen_height() as f32,
                    ),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            }
        }
        Err(_) => panic!(),
    }
}
