use bevy::prelude::*;
use bevy_steamworks::*;

fn main() {
  App::new()
    // it is important to add the plugin before `RenderPlugin` that comes with `DefaultPlugins`
    .add_plugins(SteamworksPlugin::init_app(480).unwrap())
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_text)
    .run();
}

fn spawn_text(
  mut commands: Commands,
) {
  commands.spawn(Camera2dBundle::default());

  commands.spawn(
    TextBundle::from_section(
      "Repro steps:\n\n1. Plug in controller.\n2. Open Steam overlay and enable Steam Input\n3. Go to templates and select generic game pad.\n4. Close steam overlay controller.\n5. Unplug controller.", TextStyle::default()
    ).with_style(Style {
      position_type: PositionType::Absolute,
      justify_self: JustifySelf::Center,
      align_self: AlignSelf::Center,
      ..default()
    })
  );
}