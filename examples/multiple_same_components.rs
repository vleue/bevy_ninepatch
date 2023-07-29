use bevy::prelude::*;

use bevy_ninepatch::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins((
            DefaultPlugins,
            // Add the `NinePatchPlugin` plugin
            NinePatchPlugin::<()>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, set_content)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
    commands.spawn(Camera2dBundle::default());

    // prepare the button
    let button_texture_handle = asset_server.load("blue_button02.png");
    let button_nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(5, 10, 6, 6));

    commands.spawn((
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(0.),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Px(0.),
                },
                width: Val::Px(300.),
                height: Val::Px(80.),

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: button_nine_patch_handle.clone(),
                texture: button_texture_handle.clone(),
                ..Default::default()
            },
            ..Default::default()
        },
        PatchElement::ButtonCancel,
    ));
    commands.spawn((
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(0.),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Px(0.),
                },
                width: Val::Px(300.),
                height: Val::Px(80.),

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: button_nine_patch_handle,
                texture: button_texture_handle,
                ..Default::default()
            },
            ..Default::default()
        },
        PatchElement::ButtonOk,
    ));
}

#[derive(Component)]
enum PatchElement {
    ButtonOk,
    ButtonCancel,
}

fn set_content(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    patch_query: Query<&PatchElement>,
    mut patch_content: Query<(Entity, &mut NinePatchContent<()>)>,
) {
    let font = asset_server.load("Kenney Future Narrow.ttf");

    for (entity, mut nine_patch_content) in &mut patch_content.iter_mut() {
        if !nine_patch_content.loaded {
            match *patch_query
                .get_component::<PatchElement>(nine_patch_content.parent)
                .expect("couldn't find tagged parent 9-Patch UI element")
            {
                PatchElement::ButtonOk => {
                    let content_entity = commands
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
                                    left: Val::Px(50.),
                                    right: Val::Auto,
                                    top: Val::Px(10.),
                                    bottom: Val::Auto,
                                },
                                ..Default::default()
                            }),
                        )
                        .id();
                    commands.entity(entity).push_children(&[content_entity]);
                    nine_patch_content.loaded = true;
                }
                PatchElement::ButtonCancel => {
                    let content_entity = commands
                        .spawn(
                            TextBundle::from_section(
                                "CANCEL",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 50.0,
                                    color: Color::RED,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect {
                                    left: Val::Px(50.),
                                    right: Val::Auto,
                                    top: Val::Px(10.),
                                    bottom: Val::Auto,
                                },
                                ..Default::default()
                            }),
                        )
                        .id();
                    commands.entity(entity).push_children(&[content_entity]);
                    nine_patch_content.loaded = true;
                }
            }
        }
    }
}
