use raylib::{
    camera::Camera3D,
    ffi::{TextureFilter, TraceLogLevel},
    init,
    math::Vector3,
    texture::{Image, RaylibTexture2D, RenderTexture2D},
    window::{get_current_monitor, get_monitor_height, get_monitor_width},
    RaylibHandle, RaylibThread,
};
pub struct RaylibData {
    pub raylib_handle: RaylibHandle,
    pub raylib_thread: RaylibThread,
}
pub struct VirtualScreen {
    pub width: u32,
    pub height: u32,
}
pub struct Settings {
    pub width: i32,
    pub height: i32,
    pub vsync: bool,
    pub msaa: bool,
    pub resizable: bool,
    pub title: String,
    pub log_level: TraceLogLevel,
}
pub struct Application {
    pub rl: RaylibData,
    pub camera: Camera3D,
    pub virtual_view: VirtualScreen,
    pub render_texture2d: RenderTexture2D,
    pub current_monitor: i32,
}
impl Application {
    pub fn new(settings: Settings, cam: Camera3D) -> Application {
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

pub enum PrimMeshes {
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
