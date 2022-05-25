#![allow(clippy::module_inception)]

mod asset_io;
mod asset_loader;
mod camera;
mod material;

use asset_io::{IronworksAssetIoPlugin, IronworksState};
use asset_loader::IronworksPlugin;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use camera::CameraPlugin;
use iyes_loopless::prelude::*;
use material::NeroMaterialPlugin;

fn main() {
	App::new()
		// Ironworks
		.add_plugins_with(DefaultPlugins, |group| {
			group.add_before::<bevy::asset::AssetPlugin, _>(IronworksAssetIoPlugin)
		})
		.add_plugin(IronworksPlugin)
		// UI
		.insert_resource(WinitSettings::desktop_app())
		.add_plugin(EguiPlugin)
		.add_system(ui_need_ironworks_resource.run_not_in_state(IronworksState::Ready))
		.add_system(ui_main.run_in_state(IronworksState::Ready))
		// 3D
		.add_plugin(CameraPlugin)
		.add_plugin(NeroMaterialPlugin)
		// Asset test stuff
		.add_enter_system(IronworksState::Ready, asset_test)
		// Done
		.run();
}

fn asset_test(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	// mut meshes: ResMut<Assets<Mesh>>,
	// mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// 2D texture test
	// commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	// commands.spawn_bundle(SpriteBundle {
	// 	texture: asset_server.load("iw://bg/ffxiv/sea_s1/twn/common/texture/s1t0_a0_flag1_d.tex"),
	// 	..default()
	// });

	// 3D model test
	// commands
	// 	.spawn_scene(asset_server.load("iw://bg/ffxiv/sea_s1/twn/common/bgparts/s1t0_z0_flg3.mdl"));
	// commands
	// 	.spawn_scene(asset_server.load("iw://bg/ffxiv/sea_s1/twn/s1ta/bgparts/s1ta_ga_char1.mdl"));
	// commands
	// 	.spawn_scene(asset_server.load("iw://bg/ffxiv/sea_s1/twn/s1ta/bgparts/s1ta_ga_flr2.mdl"));
	commands
		.spawn_scene(asset_server.load("iw://bg/ffxiv/wil_w1/dun/w1d5/bgparts/w1d5_q1_bre4b.mdl"));
	commands.spawn_bundle(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
}

fn ui_need_ironworks_resource(
	mut commands: Commands,
	mut egui_context: ResMut<EguiContext>,
	ironworks_state: Res<CurrentState<IronworksState>>,
) {
	let pending = *ironworks_state == CurrentState(IronworksState::ResourceRequested);

	egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
		ui.vertical_centered(|ui| {
			ui.heading("nero");

			// TODO: Work out how to show errors from path validation.
			ui.label("Could not find game installation path.");

			if ui
				.add_enabled(!pending, egui::Button::new("Select game folder"))
				.clicked()
			{
				commands.insert_resource(NextState(IronworksState::ResourceRequested));
			}
		})
	});
}

struct TempImages {
	logo: Handle<Image>,
}

impl FromWorld for TempImages {
	fn from_world(world: &mut World) -> Self {
		let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
		Self {
			logo: asset_server.load("logo.png"),
		}
	}
}

fn ui_main(
	mut egui_context: ResMut<EguiContext>,
	mut temp_logo: Local<egui::TextureId>,
	mut temp_init: Local<bool>,
	temp_images: Local<TempImages>,
) {
	if !*temp_init {
		*temp_init = true;
		*temp_logo = egui_context.add_image(temp_images.logo.clone_weak());
	}

	let ctx = egui_context.ctx_mut();

	// TODO: this would probably be managed by a controller or something
	egui::SidePanel::left("tabs")
		.width_range(20.0..=20.0)
		.resizable(false)
		.frame(egui::Frame::default().fill(ctx.style().visuals.window_fill()))
		.show(ctx, |ui| {
			let button = egui::Button::image_and_text(*temp_logo, [34.0, 50.0], "");
			ui.add(button);
		});

	// TODO: anything beyond the initial tabs should be a concern of the active tab. traits?
	egui::SidePanel::left("explorer")
		.resizable(true)
		.show(ctx, |ui| {
			ui.heading("explorer");
		});
}
