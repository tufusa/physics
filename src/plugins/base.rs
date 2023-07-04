use bevy::{prelude::*, window::*};

use crate::config;

impl Plugin for super::Base {
    #[cfg(not(target_family = "wasm"))]
    fn build(&self, app: &mut App) {
        use bevy_embedded_assets::EmbeddedAssetPlugin;

        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::Windowed,
                        title: config::Window::TITLE.into(),
                        resolution: config::Window::SIZE.into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
        );
    }

    #[cfg(target_family = "wasm")]
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                title: config::Title::TITLE.into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}
