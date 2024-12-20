use raylib::{
    ffi::ShaderLocationIndex,
    math::{rvec2, Vector4},
    models::{Mesh, Model, RaylibMesh},
    shaders::{RaylibShader, Shader},
    RaylibHandle, RaylibThread,
};

use crate::structs::{Application, PrimMeshes, VirtualScreen};

pub fn load_light_shader(raylib_handle: &mut RaylibHandle, raylib_thread: &RaylibThread, ambience: Vector4) -> Shader {
    let mut shader = raylib_handle.load_shader(
        raylib_thread,
        Some("./shader/simpleLight.vs"),
        Some("./shader/simpleLight.fs"),
    );

    shader.locs_mut()[ShaderLocationIndex::SHADER_LOC_MATRIX_MODEL as usize] =
        shader.get_shader_location("matModel");

    shader.locs_mut()[ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize] =
        shader.get_shader_location("viewPos");

    let amb = shader.get_shader_location("ambient");
    shader.set_shader_value(amb, [ambience.x, ambience.y, ambience.z, ambience.w]);
    shader
}

pub fn gen_prim_mesh(shape: PrimMeshes, raylib_thread: &RaylibThread) -> Mesh {
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

pub fn mesh_to_model(
    raylib_handle: &mut RaylibHandle,
    raylib_thread: &RaylibThread,
    mesh: Mesh,
) -> Model {
    raylib_handle
        .load_model_from_mesh(raylib_thread, unsafe { mesh.make_weak() })
        .unwrap()
}

pub fn get_screen_scale(virtual_screen: &VirtualScreen, handle: &RaylibHandle) -> f32 {
    (handle.get_screen_width() as f32 / virtual_screen.width as f32)
        .min(handle.get_screen_height() as f32 / virtual_screen.height as f32)
}
pub fn set_mouse(virtual_screen: &VirtualScreen, screen_scale: &f32, handle: &mut RaylibHandle) {
    handle.set_mouse_scale(1.0 / screen_scale, 1.0 / screen_scale);
    handle.set_mouse_offset(rvec2(
        -(handle.get_screen_width() as f32 - (virtual_screen.width as f32 * screen_scale)) * 0.5,
        -(handle.get_screen_height() as f32 - (virtual_screen.height as f32 * screen_scale)) * 0.5,
    ));
}
