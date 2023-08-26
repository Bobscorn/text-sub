use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::*;
use crate::components::*;
use crate::constants::*;
use crate::events::*;
use crate::enums::*;
use crate::resources::*;
use super::ui::*;

pub fn setup_mainmenu(
    mut commands: Commands,
    colors: Res<Colors>,
    fonts: Res<FontResource>
) {
    let root = commands.spawn(NodeBundle{
        style: Style {
            size: Size { width: Val::Percent(100.0), height: Val::Percent(100.0) },
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            gap: Size { width: Val::Px(0.0), height: Val::Px(25.0) },
            ..default()
        },
        background_color: colors.menu_background.into(),
        ..default()
    }).with_children(|node_parent| {
        node_parent.spawn((ButtonBundle {
                style: Style {
                    size: Size { width: Val::Px(225.0), height: Val::Px(50.0) },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: colors.button_normal.into(),
                ..default()
            },
            InteractButton::from_clicked(colors.button_normal, colors.button_pressed),
            MyButton{ identifier: ButtonType::PlayButton })
        ).with_children(|button_parent| {
            button_parent.spawn(TextBundle::from_section("Play", TextStyle{ 
                font: fonts.font.clone(),
                font_size: 40.0,
                color: Color::rgb(0.8, 0.8, 0.8)
            }));
        });
        node_parent.spawn((ButtonBundle{ 
                style: Style {
                    size: Size { width: Val::Px(225.0), height: Val::Px(50.0) },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: colors.button_normal.into(),
                ..default()
            }, 
            InteractButton::from_clicked(colors.button_normal, colors.button_pressed), 
            MyButton{ identifier: ButtonType::SubBuilderButton })
        ).with_children(|button_parent| {
            button_parent.spawn(TextBundle::from_section("sub Builder", TextStyle{
                font: fonts.font.clone(),
                font_size: 40.0,
                color: Color::rgb(0.8, 0.8, 0.8)
            }));
        });
    }).id();

    commands.insert_resource(UIMenu{ ui: root });
}

pub fn exit_main_menu(
    mut commands: Commands,
    menu_res: Res<UIMenu>
) {
    if let Some(ent) = commands.get_entity(menu_res.ui) {
        ent.despawn_recursive();
    }
    commands.remove_resource::<UIMenu>();
}

pub fn handle_menu_buttons(
    interaction_query: Query<
        (
            &Interaction,
            &MyButton
        ), (
            Changed<Interaction>, 
            With<Button>
        )
    >,
    mut next_state: ResMut<NextState<GameState>>
) {
    for (interaction, button_type) in &interaction_query {
        if *interaction != Interaction::Clicked {
            continue;
        }

        match button_type.identifier {
            ButtonType::PlayButton => {
                info!("Play button pressed, going into matchmaking.");
                next_state.set(GameState::MatchMaking);
            },
            ButtonType::SubBuilderButton => {
                info!("sub builder button pressed, going into.... something......");
                next_state.set(GameState::SubBuilding);
            },
            ButtonType::BackToMenuButton => {
                info!("Back to Menu button pressed, going back.");
                next_state.set(GameState::MainMenu);
            }
        }
    }
}

pub fn setup_sub_builder(
    mut commands: Commands,
    fonts: Res<FontResource>,
    colors: Res<Colors>,
    sub: Res<Submarine>
) {
    //
    let mut root_commands = commands.spawn(
        NodeBundle{
            style: Style {
                size: Size{ width: Val::Percent(100.0), height: Val::Percent(100.0) },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }
    );

    // Back Button
    // README: To make UI work with multiple screen/window sizes, use a mix of the flex 
    // sizing stuff (margin, padding, border, flex_shrink, flex_grow) and the min/max sizing stuff
    root_commands.with_children(|root_parent| {
        root_parent.spawn(
            NodeBundle{
                style: Style {
                    size: Size{ width: Val::Percent(100.0), height: Val::Percent(10.0) },
                    align_content: AlignContent::Start,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                ..default()
            }
        ).with_children(|node_parent| {
            node_parent.spawn(
                (
                    ButtonBundle {
                        style: Style {
                            size: Size{ width: Val::Px(50.0), height: Val::Px(25.0) },
                            ..default()
                        },
                        background_color: colors.button_normal.into(),
                        ..default()
                    },
                    MyButton { identifier: ButtonType::BackToMenuButton },
                    InteractButton::from_clicked(colors.button_normal, colors.button_pressed)
                )
            ).with_children(|button_parent| {
                button_parent.spawn(TextBundle::from_section("Back", TextStyle {
                    font: fonts.font.clone(),
                    font_size: 20.0,
                    color: colors.normal_text
                }));
            });
        });
    });

    //spawn builder UI
    root_commands.with_children(|root_parent| {
        root_parent.spawn( //Builder UI Container
            NodeBundle{ 
                style: Style { //Builder UI Style
                    size: Size { width: Val::Percent(100.0), height: Val::Percent(90.0) },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::End,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                ..default()
            }
        ).with_children(|root_parent| { 
            root_parent.spawn(NodeBundle { //Buttons Container
                style: Style {
                    size: Size { width: Val::Percent(100.0), height: Val::Percent(20.0) },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                background_color: colors.node_background.into(),
                ..default()
            }).with_children(|node_parent| {
                for piece in SUB_PARTS {
                    node_parent.spawn( //button
                        (ButtonBundle {
                            style: Style {
                                size: Size { width: Val::Px(40.0), height: Val::Px(40.0) },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: colors.button_normal.into(),
                            ..default()
                        }, 
                        SubBuilderButton{ part: piece },
                        InteractButton::from_clicked(colors.button_normal, colors.button_pressed)
                    ))
                    .with_children(|button_parent| { //button label
                        button_parent.spawn(
                            TextBundle{ text: Text::from_section(piece.symbol, fonts.p1_font.clone()), ..default() }
                        );
                    })
                    .with_children(|root| { //hover text
                        root.spawn(
                            TextBundle {text: Text::from_section(piece.label, fonts.p1_font.clone()), visibility: Visibility::Hidden, ..default()}
                        );
                    });
                }
            });
        });
    });

    let root_id = root_commands.id();
    commands.insert_resource(UIMenu{ ui: root_id });

    // Spawn the placement grid

    // TODO: that ^

    // Temporarily insert sub builder preview resource
    let preview_ent = commands.spawn(Text2dBundle{
        text: Text::from_section("$", fonts.preview_font.clone()),
        transform: Transform::from_translation(Vec3::new(-4000.0, 0.0, 2.0)).with_scale(Vec3::ONE * SUB_SCALE),
        ..default()
    }).id();

    commands.insert_resource(
        SubBuilderPreview{ 
            ent: preview_ent, 
            piece: &REACTOR,
            rotation: PieceRotation::North
        }); 


    // Add the subbuildersub resource
    // Copy entities from the sub resource
    let mut builder_sub = SubBuilder::default();
    builder_sub.root = commands.spawn(SpatialBundle::default()).with_children(|root| {

        let left: i32 = -25;
        let bottom: i32 = -20;
        for x in 0..SUB_MAX_WIDTH {
            for y in 0..SUB_MAX_HEIGHT {
                if sub.pieces[x][y] == EMPTY_CHAR {
                    continue;
                }

                let pos = Vec3::new((x as i32 + left) as f32 * SUB_STRUCTURE_SPACING, (y as i32 + bottom) as f32 * SUB_STRUCTURE_SPACING, 0.0);
                info!("Spawning '{}' at {:?} grid: ({}, {})", sub.pieces[x][y], pos, x, y);

                let ent = root.spawn(
                    Text2dBundle{
                        text: Text::from_section(sub.pieces[x][y], fonts.p1_font.clone()),
                        transform: Transform::from_scale(Vec3::ONE * SUB_SCALE).with_translation(pos).with_rotation(Quat::from_rotation_z(sub.rotations[x][y].rotation_radians())),
                        ..default()
                        }
                ).id();

                builder_sub.pieces[x][y] = Some(ent);
            }
        }
    }).id();

    commands.insert_resource(builder_sub);

}

pub fn exit_sub_builder(
    mut commands: Commands,
    builder_sub: Res<SubBuilder>,
    preview: Res<SubBuilderPreview>,
    menu: Res<UIMenu>
) {
    if let Some(root_entity) = commands.get_entity(menu.ui) {
        root_entity.despawn_recursive();
    }

    commands.remove_resource::<UIMenu>();

    if let Some(e_coms) = commands.get_entity(builder_sub.root) {
        e_coms.despawn_recursive();
    }
    commands.remove_resource::<SubBuilder>();

    if let Some(e_coms) = commands.get_entity(preview.ent) {
        e_coms.despawn_recursive();
    }
    commands.remove_resource::<SubBuilderPreview>();
}

pub fn sub_builder_navigation_buttons(
    colors: Res<Colors>,
    interaction_query: Query<
            (
                &Interaction,
                &MyButton
            ),
            (Changed<Interaction>, With<Button>)
        >,
    mut next_state: ResMut<NextState<GameState>>
) {

}

pub fn sub_builder_piece_buttons(
    mut preview: ResMut<SubBuilderPreview>,
    button_query: Query<
        (
            &Interaction,
            &SubBuilderButton,
            &Children
        ),
        (Changed<Interaction>, With<Button>)
    >,
    mut text_query: Query<&mut Text, (Without<Button>, Without<Interaction>, Without<SubBuilderButton> )>,
    mut visibility_query: Query<&mut Visibility, (Without<Button>, Without<Interaction>, Without<SubBuilderButton> )>
) {
    for (interaction, builder_button, children) in &button_query {
        match *interaction {
            Interaction::Clicked => {
                if let Ok(mut text) = text_query.get_mut(preview.ent) {
                    text.sections[0].value = String::from(builder_button.part.symbol);
                    preview.piece = builder_button.part;
                }
            },
            Interaction::Hovered => apply_visibility(&mut visibility_query, children, Visibility::Visible),
            Interaction::None => apply_visibility(&mut visibility_query, children, Visibility::Hidden)
        }
    }
}

fn apply_visibility(query: &mut Query<&mut Visibility, (Without<Button>, Without<Interaction>, Without<SubBuilderButton> )>, 
    children: &Children, input: Visibility) {

    let hover_entity = children.get(1).unwrap();
    let owned_hover_entity = hover_entity.to_owned();
    
    let mut vis_comp = match query.get_mut(owned_hover_entity) {
        Ok(item) => item,
        Err(_) => panic!("error: Could not find Hover Text Component!"),
    };
    *vis_comp = input;
}

pub fn rotate_sub_preview(
    mut preview: ResMut<SubBuilderPreview>,
    mut query: Query<&mut Transform>,
    input: Res<Input<KeyCode>>
) {
    let rotate_cw = KeyCode::E;
    let rotate_ccw = KeyCode::Q;

    if !input.any_pressed([rotate_ccw, rotate_cw]) {
        return;
    }

    if input.just_pressed(rotate_cw) {
        let mut transform = match query.get_mut(preview.ent) {
            Ok(e) => e,
            Err(_) => return
        };

        transform.rotate_local_z(PI * -0.5);
        preview.rotation = preview.rotation.rotated_cw();
    }
    if input.just_pressed(rotate_ccw) {
        let mut transform = match query.get_mut(preview.ent) {
            Ok(e) => e,
            Err(_) => return
        };

        transform.rotate_local_z(PI * 0.5);
        preview.rotation = preview.rotation.rotated_ccw();
    }
}

pub fn do_sub_builder_parts(
    mut commands: Commands,
    // For converting coordinates v
    window_query: Query<&Window, With<PrimaryWindow>>,
    cam_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // For getting and moving preview v
    preview_opt: Option<Res<SubBuilderPreview>>,
    mut trans_query: Query<&mut Transform, (Without<Camera>, Without<Window>)>,
    // For destroying and placing pieces v
    input: Res<Input<MouseButton>>,
    fonts: Res<FontResource>,
    over_ui: Res<UiHandling>,
    mut sub: ResMut<Submarine>,
    mut subbuilder_sub: ResMut<SubBuilder>,
) {
    if over_ui.is_pointer_over_ui {
        return;
    }

    // Convert mouse position to world position
    // v

    let (cam, cam_trans) = match cam_query.get_single() {
        Ok((c, ct)) => (c, ct),
        Err(_) => return
    };

    let window = match window_query.get_single() {
        Ok(w) => w,
        Err(_) => return
    };

    let cursor_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return
    };

    let camera_ray = match cam.viewport_to_world(cam_trans, cursor_pos) {
        Some(r) => r,
        None => return
    };

    let world_pos = camera_ray.origin.truncate();
    let grid_pos = Vec2::new(world_pos.x / SUB_STRUCTURE_SPACING, world_pos.y / SUB_STRUCTURE_SPACING).round();

    // Round to the nearest grid position
    let world_pos = Vec2::new(
        grid_pos.x * SUB_STRUCTURE_SPACING, 
        grid_pos.y * SUB_STRUCTURE_SPACING
    );
    // ^
    // Convert mouse position to world position

    let place_key = MouseButton::Left;
    let destroy_key = MouseButton::Right;

    // Do Piece destruction
    // v
    if input.pressed(destroy_key) {
        let left = -25;
        let bottom = -20;
        let x = (grid_pos.x as i32 - left).clamp(0, SUB_MAX_WIDTH as i32 - 1) as usize;
        let y = (grid_pos.y as i32 - bottom).clamp(0, SUB_MAX_HEIGHT as i32 - 1) as usize;

        if let Some(ent) = subbuilder_sub.pieces[x][y] {
            if let Some(e_coms) = commands.get_entity(ent) {
                e_coms.despawn_recursive();
            }
            subbuilder_sub.pieces[x][y] = None;
            info!("Deleting sub builder piece '{}' at: {:?}", sub.pieces[x][y], world_pos);
            sub.pieces[x][y] = '\0';
        }
    }
    // ^
    // Piece destruction

    // Move Preview
    // v
    
    let preview = match preview_opt {
        Some(p) => p,
        None => return // return as piece placement depends on preview existing
    };

    let mut preview_trans = match trans_query.get_mut(preview.ent) {
        Ok(t) => t,
        Err(_) => return
    };

    preview_trans.translation = world_pos.extend(2.0);
    // ^
    // Move Preview

    // Piece Placement v
    if input.pressed(place_key) {
        let left = -25;
        let bottom = -20;
        let x = (grid_pos.x as i32 - left).clamp(0, SUB_MAX_WIDTH as i32 - 1) as usize;
        let y = (grid_pos.y as i32 - bottom).clamp(0, SUB_MAX_HEIGHT as i32 - 1) as usize;

        if sub.pieces[x][y] != EMPTY_CHAR {
            return;
        }

        info!("Placing piece '{}' at {:?}", preview.piece.symbol, world_pos);
        let piece = commands.spawn(
            Text2dBundle
            { 
                transform: Transform::from_scale(Vec3::ONE * SUB_SCALE).with_translation(world_pos.extend(0.0)).with_rotation(preview_trans.rotation),
                text: Text::from_section(preview.piece.symbol, fonts.p1_font.clone()),
                ..default()
            }).id();
        
        // Make the spawned piece a child of the subbuilder root so it'll be destroyed when destroying the root
        if let Some(mut e_coms) = commands.get_entity(subbuilder_sub.root) {
            e_coms.add_child(piece);
        }

        subbuilder_sub.pieces[x][y] = Some(piece);
        sub.pieces[x][y] = preview.piece.symbol;
        sub.rotations[x][y] = preview.rotation;
    }
    // ^
    // Piece Placement
}
