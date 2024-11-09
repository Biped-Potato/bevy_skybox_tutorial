use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        texture::{ImageSampler, ImageSamplerDescriptor},
    }, window::{CursorGrabMode, PrimaryWindow},
};

pub mod camera_controller;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_controller::update_camera_controller,reinterpret_cubemap))
        .run();
}

#[derive(Resource)]
pub struct SkyCubeMap {
    pub image: Handle<Image>,
    pub loaded: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>,mut window_query : Query<&mut Window,With<PrimaryWindow>>) {
    let sky_image = asset_server.load("skysheet.png");

    let mut window = window_query.get_single_mut().unwrap();
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;

    commands.insert_resource(SkyCubeMap {
        image: sky_image.clone(),
        loaded: false,
    });
    commands.spawn((
        Camera3dBundle {
            transform: Transform::IDENTITY,
            ..Default::default()
        },
        camera_controller::CameraController{
            sensitivity : 0.035,
            rotation : Vec2::ZERO,
            rotation_lock : 88.0,
        },
        Skybox {
            image: sky_image.clone(),
            brightness: 1000.,
        },
    ));
}

pub fn reinterpret_cubemap(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<SkyCubeMap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.loaded && asset_server.load_state(&cubemap.image) == LoadState::Loaded {
        cubemap.loaded = true;
        let image = images.get_mut(&cubemap.image).unwrap();

        if image.texture_descriptor.array_layer_count() == 1 {
            //6
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..Default::default()
            });
        }
        //set all skybox images to the new array texture
        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image.clone();
        }
    }
}
