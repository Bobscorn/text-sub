use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::*;
use bevy_pkv::*;

use crate::components::*;
use crate::constants::*;
use crate::events::*;
use crate::enums::*;
use crate::resources::*;
use super::ui::*;
use crate::structs::*;
use crate::parts::*;

pub fn setup_mainmenu(
    mut commands: Commands,
    colors: Res<Colors>,
    fonts: Res<FontResource>
) {
    let root = commands.spawn(NodeBundle{
        style: Style {
            width: Val::Percent(100.0), 
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(25.0),
            ..default()
        },
        background_color: colors.menu_background.into(),
        ..default()
    }).with_children(|node_parent| {
        node_parent.spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(225.0), 
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::Px(10.),
                        bottom: Val::Px(10.)
                    },
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
                    width: Val::Px(225.0), 
                    height: Val::Px(50.0),
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
            button_parent.spawn(TextBundle::from_section("Sub Builder", TextStyle{
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
    mut next_state: ResMut<NextState<GameState>>,
    mut storage: ResMut<PkvStore>,
    submarine: Res<Submarine>,
) {
    for (interaction, button_type) in &interaction_query {
        if *interaction != Interaction::Pressed {
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
            },
            ButtonType::SaveButton => {
                info!("Save Button pressed");
                let derefed_sub: &Submarine = &submarine;
                storage.set(CACHED_KEY, derefed_sub).expect("failed to store saved_build");
            }
        }
    }
}

pub fn setup_sub_builder(
    mut commands: Commands,
    fonts: Res<FontResource>,
    colors: Res<Colors>,
    sub: Res<Submarine>,
    mut submarine: ResMut<PkvStore>
) {
    
    let sub_parts: [&SubPart; 12] = [
        &ARMOUR1, &ARMOUR2, &ARMOUR3, &ARMOUR4, &ARMOUR5, &ARMOUR6, &ARMOUR7, &ARMOUR8, &ARMOUR9,
        &PROPELLER1, &TORPEDO_LAUNCHER, &REACTOR
    ];

    let mut root_commands = commands.spawn(
        NodeBundle{
            style: Style {
                width: Val::Percent(100.0), 
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }
    );

    // Navigation buttons
    // README: To make UI work with multiple screen/window sizes, use a mix of the flex 
    // sizing stuff (margin, padding, border, flex_shrink, flex_grow) and the min/max sizing stuff
    root_commands.with_children(|root_parent| {
        root_parent.spawn(
            NodeBundle{
                style: Style {
                    width: Val::Percent(100.0), 
                    height: Val::Percent(10.0),
                    align_content: AlignContent::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::Px(10.),
                        bottom: Val::Px(10.)
                    },
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|node_parent| {
            node_parent.spawn(
                (
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(50.0), 
                            height: Val::Px(25.0),
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
        })
        .with_children(|node_parent| {
            node_parent.spawn(
                (
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(50.0), 
                            height: Val::Px(25.0),
                            ..default()
                        },
                        background_color: colors.button_normal.into(),
                        ..default()
                    },
                    MyButton { identifier: ButtonType::SaveButton },
                    InteractButton::from_clicked(colors.button_normal, colors.button_pressed)
                )
            ).with_children(|button_parent| {
                button_parent.spawn(TextBundle::from_section("Save", TextStyle { //save button
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
                    width: Val::Percent(100.0), 
                    height: Val::Percent(90.0),
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
                    width: Val::Percent(100.0), 
                    height: Val::Percent(20.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                background_color: colors.node_background.into(),
                ..default()
            }).with_children(|node_parent| {
                for part in sub_parts {
                    node_parent.spawn(( //symbol button
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(40.0), 
                                    height: Val::Px(100.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..default()
                                },
                                background_color: colors.node_background.into(),                            
                                ..default()
                            }, 
                            SubBuilderButton{ part: part.clone() },
                            InteractButton::from_clicked(colors.node_background, colors.button_pressed),
                            SymbolButton {}
                    ))
                    .with_children(|root| { //hover text
                        root.spawn((
                            TextBundle {text: Text::from_section(part.label, fonts.p1_font.clone()), background_color: colors.node_background.into(), 
                                visibility: Visibility::Hidden, z_index: ZIndex::Global(5),
                                ..default()},
                            HoverText {}
                        ));
                    })
                    .with_children(|button_parent| { //button label
                        button_parent.spawn(
                            TextBundle{ text: Text::from_section(part.symbol, fonts.p1_font.clone()), background_color: colors.node_background.into(), 
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                ..default() }
                        );
                    });
                }
            });
        });
    });

    let root_id = root_commands.id();
    commands.insert_resource(UIMenu{ ui: root_id });

    // Spawn the placement grid

    let rows = 20;
    let columns = 20;
    let column_height = 20.0;
    let row_width = 20.0;
    let x_start = Vec3::new(-(SUB_STRUCTURE_SPACING * (columns as f32 + 0.5) * 0.5), 0.0, 2.0); //divide the structure spacing by 2
    let y_start = Vec3::new(0.0, -(SUB_STRUCTURE_SPACING * (rows as f32 + 0.5) * 0.5), 2.0);
    for x in 0..columns {
        commands.spawn(SpriteBundle{
            sprite: Sprite { 
                color: Color::rgb(0.85, 0.85, 0.85), 
                custom_size: Some(Vec2::new(2.0, column_height)), 
                ..default() 
            },
            transform: Transform::from_translation(x_start + Vec3::new(SUB_STRUCTURE_SPACING, 0.0, 0.0) * (x as f32)),
            ..default()
        });
    }
    for y in 0..rows {
        commands.spawn(SpriteBundle{
            sprite: Sprite { 
                color: Color::rgb(0.85, 0.85, 0.85), 
                custom_size: Some(Vec2::new(row_width, 2.0)), 
                ..default() 
            },
            transform: Transform::from_translation(y_start + Vec3::new(0.0, SUB_STRUCTURE_SPACING, 0.0) * (y as f32)),
            ..default()
        });
    }

    // Temporarily insert sub builder preview resource
    let preview_ent = commands.spawn(Text2dBundle{
        text: Text::from_section("$", fonts.preview_font.clone()),
        transform: Transform::from_translation(Vec3::new(-4000.0, 0.0, 2.0)).with_scale(Vec3::ONE * SUB_SCALE),
        ..default()
    }).id();

    
    commands.insert_resource(
        SubBuilderPreview{ 
            entity: preview_ent, 
            part: REACTOR.clone(),
            rotation: PieceRotation::default()
        }); 


    // Add the subbuildersub resource
    // Copy entities from the sub resource
    let mut builder_sub = SubBuilder::default();
    builder_sub.root = commands.spawn(SpatialBundle::default()).with_children(|root| {

        let left: i32 = -25;
        let bottom: i32 = -20;
        
        builder_sub.parts.clear();

        for x in 0..sub.parts.len() {
            let column = &sub.parts[x];
            let mut builder_column = Vec::new();

            for y in 0..column.len() {
                let symbol = sub.parts[x][y];

                if symbol == EMPTY_CHAR {
                    builder_column.push(None);
                    continue;
                }

                let rotations = sub.rotations[x][y];

                let pos = Vec3::new((x as i32 + left) as f32 * SUB_STRUCTURE_SPACING, (y as i32 + bottom) as f32 * SUB_STRUCTURE_SPACING, 0.0);
                info!("Spawning '{symbol}' at {pos:?} grid: ({x}, {y})");

                let ent = root.spawn(
                    Text2dBundle{
                        text: Text::from_section(symbol.to_string(), fonts.p1_font.clone()),
                        transform: Transform::from_scale(Vec3::ONE * SUB_SCALE).with_translation(pos).with_rotation(Quat::from_rotation_z(rotations.rotation_radians())),
                        ..default()
                        }
                ).id();

                builder_column.push(Some(ent));
            }
            builder_sub.parts.push(builder_column);
        }
    })
    .id();

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

    if let Some(e_coms) = commands.get_entity(preview.entity) {
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

pub fn sub_builder_part_buttons(
    mut preview: ResMut<SubBuilderPreview>,
    button_query: Query<
        (&Interaction, &SubBuilderButton, &Children),
        (Changed<Interaction>, With<Button>, With<SymbolButton>)>,
    mut text_query: Query<&mut Text, (With<SymbolButton>, Without<Button>, Without<Interaction>, Without<SubBuilderButton> )>,
    mut visibility_query: Query<&mut Visibility, (With<HoverText>, Without<Button>, Without<Interaction>, Without<SubBuilderButton> )>
) {
    for (interaction, builder_button, children) in &button_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(mut text) = text_query.get_mut(preview.entity) {
                    text.sections[0].value = String::from(builder_button.part.symbol);
                    preview.part = builder_button.part;
                }
            },
            Interaction::Hovered => apply_visibility(&mut visibility_query, children, Visibility::Visible),
            Interaction::None => apply_visibility(&mut visibility_query, children, Visibility::Hidden)
        }
    }
}

fn apply_visibility(query: &mut Query<&mut Visibility, (With<HoverText>, Without<Button>, Without<Interaction>, Without<SubBuilderButton> )>, 
    children: &Children, input: Visibility) {

    let hover_entity = children.get(0).unwrap();
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
        let mut transform = match query.get_mut(preview.entity) {
            Ok(e) => e,
            Err(_) => return
        };

        transform.rotate_local_z(PI * -0.5);
        preview.rotation = preview.rotation.rotated_cw();
    }
    if input.just_pressed(rotate_ccw) {
        let mut transform = match query.get_mut(preview.entity) {
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
    // For destroying and placing sub parts v
    input: Res<Input<MouseButton>>,
    fonts: Res<FontResource>,
    over_ui: Res<UiHandling>,
    mut sub: ResMut<Submarine>,
    mut subbuilder: ResMut<SubBuilder>,
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

    // v Do part destruction
    if input.pressed(destroy_key) {
        let left = -25;
        let bottom = -20;
        let x = (grid_pos.x as i32 - left).clamp(0, SUB_MAX_WIDTH as i32 - 1) as usize;
        let y = (grid_pos.y as i32 - bottom).clamp(0, SUB_MAX_HEIGHT as i32 - 1) as usize;

        if let Some(ent) = subbuilder.parts[x][y] {
            if let Some(e_coms) = commands.get_entity(ent) {
                e_coms.despawn_recursive();
            }
            subbuilder.parts[x][y] = None;
            info!("Deleting sub builder part '{}' at: {:?}", sub.parts[x][y], world_pos);
            sub.parts[x][y] = '\0';
        }
    }
    // ^ part destruction

    // v Move Preview
    
    let preview = match preview_opt {
        Some(p) => p,
        None => return // return as part placement depends on preview existing
    };

    let mut preview_trans = match trans_query.get_mut(preview.entity) {
        Ok(t) => t,
        Err(_) => return
    };

    preview_trans.translation = world_pos.extend(2.0);
    // ^ Move Preview

    // part Placement v
    if input.pressed(place_key) {
        let left = -25;
        let bottom = -20;
        
        //derive grid coordinates
        let x: usize = (grid_pos.x as i32 - left).clamp(0, SUB_MAX_WIDTH as i32 - 1) as usize;
        let y = (grid_pos.y as i32 - bottom).clamp(0, SUB_MAX_HEIGHT as i32 - 1) as usize;

        if sub.parts.len() <= x {
            let parts_columns_len = sub.parts.len() as i32;
            let diff = (x as i32) - parts_columns_len + 1;

            for _index in 0..diff {
                sub.parts.push(Vec::new());
                sub.rotations.push(Vec::new());
                subbuilder.parts.push(Vec::new());
            }
        }

        if sub.parts[x].len() <= y {
            let parts_rows_len = sub.parts[x].len() as i32;
            let diff = (y as i32) - parts_rows_len + 1;

            for _index in 0..diff {
                sub.parts[x].push(EMPTY_CHAR);
                sub.rotations[x].push(PieceRotation::default());
                subbuilder.parts[x].push(None);
            }
        }

        if sub.parts[x][y] != EMPTY_CHAR { //conclude if the grid slot was taken
            return;
        }

        info!("Placing part '{}' at {:?}", preview.part.symbol, world_pos);
        
        let part = commands.spawn(
            Text2dBundle
            { 
                transform: Transform::from_scale(Vec3::ONE * SUB_SCALE).with_translation(world_pos.extend(0.0)).with_rotation(preview_trans.rotation),
                text: Text::from_section(preview.part.symbol, fonts.p1_font.clone()),
                ..default()
            }).id();
        
        // Make the spawned part a child of the subbuilder root so it'll be destroyed when destroying the root
        if let Some(mut e_coms) = commands.get_entity(subbuilder.root.clone()) {
            e_coms.add_child(part);
        }

        subbuilder.parts[x][y] = Some(part);
        sub.parts[x][y] = preview.part.symbol;
        sub.rotations[x][y] = preview.rotation;
    }
    // ^
    // part Placement
}

