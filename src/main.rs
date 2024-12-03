#![allow(dead_code, unused_variables)]

use raylib::prelude::*;

mod structs;
use structs::*;

mod constants;
use constants::*;

mod util;

use util::*;

fn main() {
    let mut application = Application::new(
        Settings {
            width: INITIAL_WINDOW_WIDTH,
            height: INITIAL_WINDOW_HEIGHT,
            vsync: true,
            msaa: true,
            resizable: true,
            title: "Window".to_string(),
            log_level: Default::default(),
        },
        Camera3D::perspective(rvec3(0, 2, 4), rvec3(0, 0, 0), rvec3(0, 1, 0), 45.0),
    );

    let mut shader = load_light_shader(&mut application);

    let cube_mesh = gen_prim_mesh(
        PrimMeshes::Cube {
            width: 1.0,
            height: 1.0,
            length: 1.0,
        },
        &application.rl.raylib_thread,
    );

    let mut model = mesh_to_model(
        &mut application.rl.raylib_handle,
        &application.rl.raylib_thread,
        cube_mesh,
    );
    model.materials_mut()[0].shader = *shader;
    model.materials_mut()[0].shader = *shader;

    let mut cube_position = rvec3(0, 50, 0);
    let cube_size = rvec3(5, 5, 5);

    let mut ray = Ray::default();
    let mut ray_collision = RayCollision::default();

    let collider_ground = BoundingBox {
        min: rvec3(-50, 0, -50),
        max: rvec3(50, 0, 50),
    };

    while !application.rl.raylib_handle.window_should_close() {
        let handle = &mut application.rl.raylib_handle;
        let dt = handle.get_frame_time();

        let screen_scale = get_screen_scale(&application.virtual_view, handle);

        set_mouse(&application.virtual_view, &screen_scale, handle);

        if handle.is_cursor_hidden() {
            handle.update_camera(&mut application.camera, CameraMode::CAMERA_FREE);
        }

        if handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            if handle.is_cursor_hidden() {
                handle.enable_cursor()
            } else {
                handle.disable_cursor()
            }
        }

        if handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if !ray_collision.hit {
                ray =
                    handle.get_screen_to_world_ray(handle.get_mouse_position(), application.camera);

                ray_collision = BoundingBox {
                    min: rvec3(
                        cube_position.x - cube_size.x / 2.,
                        cube_position.y - cube_size.y / 2.,
                        cube_position.z - cube_size.z / 2.,
                    ),
                    max: rvec3(
                        cube_position.x + cube_size.x / 2.,
                        cube_position.y + cube_size.y / 2.,
                        cube_position.z + cube_size.z / 2.,
                    ),
                }
                .get_ray_collision_box(ray);
            } else {
                ray_collision.hit = false;
            }
        }

        shader.set_shader_value(
            ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as i32,
            application.camera.position.x,
        );
        if collider_ground.check_collision_boxes(collider_ground) {}
        if cube_position.y > 0.0 + cube_size.y / 2. {
            cube_position.y -= dt * 10.;
        }
        //Start regular drawing
        let mut draw_handle = handle.begin_drawing(&application.rl.raylib_thread);
        draw_handle.clear_background(Color::BLACK);
        {
            //Start texture drawing , anything in here is scaled with the window size.

            let mut draw_handle = draw_handle.begin_texture_mode(
                &application.rl.raylib_thread,
                &mut application.render_texture2d,
            );
            draw_handle.clear_background(Color::LIGHTCYAN);
            {
                //Start 3D drawing
                let mut draw_handle = draw_handle.begin_mode3D(application.camera);
                draw_handle.draw_plane(rvec3(0, 0, 0), rvec2(50, 50), Color::BLACK);
                draw_handle.draw_bounding_box(collider_ground, Color::GREEN);

                if ray_collision.hit {
                    draw_handle.draw_cube_v(cube_position, cube_size, Color::RED);
                    draw_handle.draw_cube_wires_v(cube_position, cube_size, Color::MAROON);

                    draw_handle.draw_cube_wires(
                        cube_position,
                        cube_size.x + 0.2,
                        cube_size.y + 0.2,
                        cube_size.z + 0.2,
                        Color::GREEN,
                    );
                } else {
                    draw_handle.draw_cube_v(cube_position, cube_size, Color::GRAY);
                    draw_handle.draw_cube_wires_v(cube_position, cube_size, Color::DARKGRAY);
                }
                draw_handle.draw_ray(ray, Color::BLACK);
            }
            draw_handle.draw_fps(10, 10);
            draw_handle.draw_text(
                format!("screen scale {:?}", screen_scale).as_str(),
                20,
                80,
                40,
                Color::BLACK,
            );

            let mp = draw_handle.get_mouse_position();
            draw_handle.draw_circle(mp.x as i32, mp.y as i32, 6.0, Color::BLUE);
        }
        draw_handle.draw_texture_pro(
            application.render_texture2d.texture(),
            Rectangle::new(
                0.0,
                0.0,
                application.render_texture2d.texture.width as f32,
                -application.render_texture2d.texture.height as f32,
            ),
            Rectangle::new(
                0.0,
                0.0,
                draw_handle.get_screen_width() as f32,
                draw_handle.get_screen_height() as f32,
            ),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
}
