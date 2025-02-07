use bevy::prelude::*;
use bevy_steamworks::*;

fn main() {
    App::new()
      // it is important to add the plugin before `RenderPlugin` that comes with `DefaultPlugins`
      .add_plugins(SteamworksPlugin::init_app(480).unwrap())
      .add_plugins(DefaultPlugins)
      .run();
}
