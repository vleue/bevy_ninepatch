use bevy::prelude::*;

use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData, NinePatchPlugin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins((
            DefaultPlugins,
            // Add the `NinePatchPlugin` plugin
            NinePatchPlugin::<()>::default()
        ))
        .add_systems(Startup, setup)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder>>,
) {
    // prepare the text
    let font = asset_server.load("Kenney Future Narrow.ttf");

    let text_entity = commands
        .spawn(
            TextBundle::from_section(
                "OK",
                TextStyle {
                    font: font.clone(),
                    font_size: 50.0,
                    color: Color::GREEN,
                },
            )
            .with_style(Style {
                margin: UiRect {
                    left: Val::Px(60.),
                    right: Val::Auto,
                    top: Val::Px(20.),
                    bottom: Val::Auto,
                },
                ..Default::default()
            }),
        )
        .id();

    // prepare the button
    let button_texture_handle = asset_server.load("blue_button02.png");
    let button_nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(5, 10, 6, 6));

    // create a `HashMap` that will list all entities used as content of the 9-Patch UI element
    let mut button_content = std::collections::HashMap::new();
    button_content.insert((), text_entity);

    let button_entity = commands
        .spawn(
            // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
            // of this entity
            NinePatchBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Px(0.),
                        top: Val::Auto,
                        bottom: Val::Px(0.),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(200.),
                    height: Val::Px(100.),
                    ..Default::default()
                },
                nine_patch_data: NinePatchData {
                    texture: button_texture_handle,
                    nine_patch: button_nine_patch_handle,
                    content: Some(button_content),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .id();

    // prepare the panel
    let panel_texture_handle = asset_server.load("glassPanel_corners.png");
    let panel_nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20));

    commands.spawn(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(500.),
                height: Val::Px(300.),
                ..Default::default()
            },
            // helper method when there is only one content zone
            nine_patch_data: NinePatchData::with_single_content(
                panel_texture_handle,
                panel_nine_patch_handle,
                button_entity,
            ),
            ..Default::default()
        },
    );

    commands.spawn(Camera2dBundle::default());
}
