use bevy::{
    asset::AssetMetaCheck, prelude::*, reflect::TypePath, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}
};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((DefaultPlugins, Material2dPlugin::<AppearingMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, add_pixel)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<AppearingMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    // let asset:Handle<Texture> = asset_server.load("03/01_e65e10.png");

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::default().into()).into(),
        transform: Transform::default().with_scale(Vec3::splat(750.)),
        material: materials.add(
            AppearingMaterial {
                color: Color::WHITE,
                color_texture: Some(asset_server.load("03/01_e65e10.png")),
                percentage: 0.,
            }
        ),
        ..default()
    });
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for AppearingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/pixel_by_pixel.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct AppearingMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    #[uniform(3)]
    percentage: f32,
}

#[derive(Resource)]
struct AddPixelTimer(Timer);

fn add_pixel(
    mut materials: ResMut<Assets<AppearingMaterial>>
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
        materials.iter_mut().for_each(|wut|  {
            wut.1.percentage += 0.0002;
            // println!("wut.1.percentage: {}", wut.1.percentage);
            if (wut.1.percentage >= 0.5) {
                wut.1.percentage = 0.;
            }
        });
}