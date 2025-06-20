#![allow(dead_code, unused_variables)]
use std::{collections::HashMap, f32::consts::PI};
use teleia::*;

struct Assets {
    font: font::Bitmap,
    shader_flat: shader::Shader,
    shader_scene: shader::Shader,
    mesh_square: mesh::Mesh,
    texture_test: texture::Texture,
    mii: scene::Scene,
}

impl Assets {
    fn new(ctx: &context::Context) -> Self {
        Self {
            font: font::Bitmap::new(ctx),
            shader_flat: shader::Shader::new(
                ctx,
                include_str!("assets/shaders/flat/vert.glsl"),
                include_str!("assets/shaders/flat/frag.glsl"),
            ),
            shader_scene: shader::Shader::new(
                ctx,
                include_str!("assets/shaders/scene/vert.glsl"),
                include_str!("assets/shaders/scene/frag.glsl"),
            ),
            mesh_square: mesh::Mesh::from_obj(ctx, include_bytes!("assets/meshes/square.obj")),
            texture_test: texture::Texture::new(ctx, include_bytes!("assets/textures/test.png")),
            mii: scene::Scene::from_gltf(ctx, include_bytes!("assets/scenes/mii.glb")),
        }
    }
}

pub struct Game {
    assets: Assets,
}

impl Game {
    pub fn new(ctx: &context::Context) -> Self {
        Self {
            assets: Assets::new(ctx),
        }
    }
}

impl teleia::state::Game for Game {
    fn initialize_audio(&self, ctx: &context::Context, st: &state::State, actx: &audio::Context) -> HashMap<String, audio::Audio> {
        HashMap::from_iter(vec![
            ("test".to_owned(), audio::Audio::new(&actx, include_bytes!("assets/audio/test.wav"))),
        ])
    }
    fn finish_title(&mut self, _st: &mut state::State) {}
    fn mouse_press(&mut self, _ctx: &context::Context, _st: &mut state::State) {}
    fn mouse_move(&mut self, _ctx: &context::Context, _st: &mut state::State, _x: i32, _y: i32) {}
    fn update(&mut self, ctx: &context::Context, st: &mut state::State) -> Erm<()> {
        st.move_camera(
            ctx,
            &glam::Vec3::new(0.0, 0.0, -1.0),
            &glam::Vec3::new(0.0, 0.0, 1.0),
            &glam::Vec3::new(0.0, 1.0, 0.0),
        );
        Ok(())
    }
    fn render(&mut self, ctx: &context::Context, st: &mut state::State) -> Erm<()> {
        ctx.clear_color(glam::Vec4::new(0.1, 0.1, 0.1, 1.0));
        ctx.clear();
        st.bind_3d(ctx, &self.assets.shader_scene);
        self.assets.shader_scene.set_position_3d(
            ctx,
            &glam::Mat4::from_scale_rotation_translation(
                glam::Vec3::new(0.004, 0.004, 0.004),
                glam::Quat::from_euler(
                    glam::EulerRot::XYZ,
                    PI / 2.0,
                    0.0,
                    st.tick as f32 / 60.0,
                ),
                glam::Vec3::new(0.0, -0.3, 0.0),
            ),
        );
        self.assets.mii.render(ctx, &self.assets.shader_scene);
        self.assets.font.render_text(
            ctx,
            &glam::Vec2::new(0.0, 0.0),
            "hello gamerZ",
        );
        // st.bind_2d(ctx, &self.assets.shader_flat);
        // self.assets.texture_test.bind(ctx);
        // self.assets.shader_flat.set_position_2d(
        //     ctx,
        //     &glam::Vec2::new(40.0, 40.0),
        //     &glam::Vec2::new(16.0, 16.0),
        // );
        // self.assets.mesh_square.render(ctx);
        Ok(())
    }
}
