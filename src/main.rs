#![allow(dead_code, unused_variables)]

use raylib::prelude::*;

const INITIAL_WINDOW_WIDTH: i32 = 800;
const INITIAL_WINDOW_HEIGHT: i32 = 450;

struct RaylibData {
    raylib_handle: RaylibHandle,
    raylib_thread: RaylibThread,
}
struct VirtualScreen {
    width: u32,
    height: u32,
}
struct Settings {
    width: i32,
    height: i32,
    vsync: bool,
    msaa: bool,
    resizable: bool,
    title: String,
    log_level: TraceLogLevel,
}
struct Application {
    rl: RaylibData,
    camera: Camera3D,
    virtual_view: VirtualScreen,
    render_texture2d: RenderTexture2D,
    current_monitor: i32,
}
impl Application {
    fn new(settings: Settings, cam: Camera3D) -> Application {
        let mut binding = init();
        let builder = binding
            .size(settings.width, settings.height)
            .title(settings.title.as_str())
            .log_level(settings.log_level);
        if settings.vsync {
            builder.vsync();
        }
        if settings.msaa {
            builder.msaa_4x();
        }
        if settings.resizable {
            builder.resizable();
        }
        let (mut handle, thread) = builder.build();

        let monitor = get_current_monitor();

        let vw = get_monitor_width(monitor) as u32;
        let vh = get_monitor_height(monitor) as u32;

        let Ok(render_texture) = handle.load_render_texture(&thread, vw, vh) else {
            panic!("Failed to create render texture");
        };

        render_texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);

        Application {
            rl: RaylibData {
                raylib_handle: handle,
                raylib_thread: thread,
            },
            camera: cam,
            current_monitor: monitor,
            virtual_view: VirtualScreen {
                width: vw,
                height: vh,
            },
            render_texture2d: render_texture,
        }
    }
}

fn load_light_shader(application: &mut Application) -> Shader {
    let mut shader = application
        .rl
        .raylib_handle
        .load_shader(
            &application.rl.raylib_thread,
            Some("./shader/simpleLight.vs"),
            Some("./shader/simpleLight.fs"),
        );

    shader.locs_mut()[ShaderLocationIndex::SHADER_LOC_MATRIX_MODEL as usize] =
        shader.get_shader_location("matModel");

    shader.locs_mut()[ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize] =
        shader.get_shader_location("viewPos");

    let amb = shader.get_shader_location("ambient");
    shader.set_shader_value(amb, [0.2, 0.2, 0.2, 1.0]);
    shader
}

fn gen_prim_mesh(shape: PrimMeshes, raylib_thread: &RaylibThread) -> Mesh {
    match shape {
        PrimMeshes::Cube {
            width,
            height,
            length,
        } => Mesh::gen_mesh_cube(raylib_thread, width, height, length),
        PrimMeshes::CubeMap { img, size } => Mesh::gen_mesh_cubicmap(raylib_thread, &img, size),
        PrimMeshes::Cylinder {
            radius,
            height,
            slices,
        } => Mesh::gen_mesh_cylinder(raylib_thread, radius, height, slices),
        PrimMeshes::HeightMap { img, size } => Mesh::gen_mesh_heightmap(raylib_thread, &img, size),
        PrimMeshes::Hemisphere {
            radius,
            rings,
            slices,
        } => Mesh::gen_mesh_hemisphere(raylib_thread, radius, rings, slices),
        PrimMeshes::Knot {
            radius,
            size,
            rad_seg,
            sides,
        } => Mesh::gen_mesh_knot(raylib_thread, radius, size, rad_seg, sides),
        PrimMeshes::Plane {
            width,
            length,
            res_x,
            res_z,
        } => Mesh::gen_mesh_plane(raylib_thread, width, length, res_x, res_z),
        PrimMeshes::Polygon { sides, radius } => Mesh::gen_mesh_poly(raylib_thread, sides, radius),
        PrimMeshes::Sphere {
            radius,
            rings,
            slices,
        } => Mesh::gen_mesh_sphere(raylib_thread, radius, rings, slices),
        PrimMeshes::Torus {
            radius,
            size,
            rad_seg,
            sides,
        } => Mesh::gen_mesh_torus(raylib_thread, radius, size, rad_seg, sides),
    }
}
enum PrimMeshes {
    Cube {
        width: f32,
        height: f32,
        length: f32,
    },
    CubeMap {
        img: Image,
        size: Vector3,
    },
    Cylinder {
        radius: f32,
        height: f32,
        slices: i32,
    },
    HeightMap {
        img: Image,
        size: Vector3,
    },
    Hemisphere {
        radius: f32,
        rings: i32,
        slices: i32,
    },
    Knot {
        radius: f32,
        size: f32,
        rad_seg: i32,
        sides: i32,
    },
    Plane {
        width: f32,
        length: f32,
        res_x: i32,
        res_z: i32,
    },
    Polygon {
        sides: i32,
        radius: f32,
    },
    Sphere {
        radius: f32,
        rings: i32,
        slices: i32,
    },
    Torus {
        radius: f32,
        size: f32,
        rad_seg: i32,
        sides: i32,
    },
}

fn mesh_to_model(
    raylib_handle: &mut RaylibHandle,
    raylib_thread: &RaylibThread,
    mesh: Mesh,
) -> Model {
    raylib_handle
        .load_model_from_mesh(raylib_thread, unsafe { mesh.make_weak() })
        .unwrap()
}

fn get_screen_scale(virtual_screen: &VirtualScreen, handle:&RaylibHandle) -> f32{
    (handle.get_screen_width() as f32
        / virtual_screen.width as f32)
        .min(handle.get_screen_height() as f32 / virtual_screen.height as f32)
}
fn set_mouse(virtual_screen: &VirtualScreen, screen_scale: &f32, handle: &mut RaylibHandle){
    handle.set_mouse_scale(1.0 / screen_scale, 1.0 / screen_scale);
    handle.set_mouse_offset(rvec2(
        -(handle.get_screen_width() as f32
            - (virtual_screen.width as f32 * screen_scale))
            * 0.5,
        -(handle.get_screen_height() as f32
            - (virtual_screen.height as f32 * screen_scale))
            * 0.5,
    ));
}

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

    let cube_mesh = gen_prim_mesh(PrimMeshes::Cube{
        width: 1.0,
        height: 1.0,
        length: 1.0,
    }, &application.rl.raylib_thread);

    let mut model = mesh_to_model(&mut application.rl.raylib_handle, &application.rl.raylib_thread, cube_mesh);
    model.materials_mut()[0].shader = *shader;
    model.materials_mut()[0].shader = *shader;

    let mut cube_position = rvec3(0,50,0);
    let mut cube_size = rvec3(5,5,5);

    let mut ray = Ray::default();
    let mut ray_collision = RayCollision::default();

    let mut collider_ground = BoundingBox{
        min: rvec3(-50,0,-50),
        max: rvec3(50,0,50),
    };

    while !application.rl.raylib_handle.window_should_close() {
        let handle = &mut application.rl.raylib_handle;
        let dt = handle.get_frame_time();

        let screen_scale = get_screen_scale(&application.virtual_view, handle);

        set_mouse(&application.virtual_view, &screen_scale, handle);


        if handle.is_cursor_hidden() {
            handle.update_camera(&mut application.camera, CameraMode::CAMERA_FREE);
        }

        if handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT){
            if handle.is_cursor_hidden() { handle.enable_cursor() }
            else { handle.disable_cursor() }
        }

        if handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
            if !ray_collision.hit {
                ray = handle.get_screen_to_world_ray(handle.get_mouse_position(), application.camera);

                ray_collision = BoundingBox{
                    min: rvec3(cube_position.x - cube_size.x/2., cube_position.y - cube_size.y/2., cube_position.z - cube_size.z/2.),
                    max: rvec3(cube_position.x + cube_size.x/2., cube_position.y + cube_size.y/2., cube_position.z + cube_size.z/2. )
                }.get_ray_collision_box(ray);
            }
            else { ray_collision.hit = false; }

        }



        shader.set_shader_value(
            ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as i32,
            application.camera.position.x,
        );
        if collider_ground.check_collision_boxes(collider_ground){

        }
        if cube_position.y > 0.0 + cube_size.y/2.{
            cube_position.y -= dt * 10.;
        }
        //Start regular drawing
        let mut draw_handle = handle.begin_drawing(&application.rl.raylib_thread);
        draw_handle.clear_background(Color::BLACK);
        {
            //Start texture drawing , anything in here is scaled with the window size.

            let mut draw_handle = draw_handle.begin_texture_mode(&application.rl.raylib_thread, &mut application.render_texture2d);
            draw_handle.clear_background(Color::LIGHTCYAN);
            {
                //Start 3D drawing
                let mut draw_handle = draw_handle.begin_mode3D(application.camera);
                draw_handle.draw_plane(rvec3(0,0,0), rvec2(50,50), Color::BLACK);
                draw_handle.draw_bounding_box(collider_ground, Color::GREEN);

                if ray_collision.hit {
                    draw_handle.draw_cube_v(cube_position, cube_size, Color::RED);
                    draw_handle.draw_cube_wires_v(cube_position, cube_size, Color::MAROON);

                    draw_handle.draw_cube_wires(cube_position, cube_size.x + 0.2,cube_size.y + 0.2 ,cube_size.z + 0.2, Color::GREEN);
                }
                else{
                    draw_handle.draw_cube_v(cube_position, cube_size, Color::GRAY);
                    draw_handle.draw_cube_wires_v(cube_position, cube_size, Color::DARKGRAY);
                }
                draw_handle.draw_ray(ray, Color::BLACK);

            }
            draw_handle.draw_fps(10,10);
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
