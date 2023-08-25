use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::MainCamera;
use crate::constants::DOLLAR;
use crate::constants::EMPTY_CHAR;
use crate::constants::MOTHERSHIP_MAX_HEIGHT;
use crate::constants::MOTHERSHIP_MAX_WIDTH;
use crate::constants::MOTHERSHIP_SCALE;
use crate::constants::MOTHERSHIP_STRUCTURE_SPACING;
use crate::constants::SHIP_PIECES;
use crate::events::*;
use crate::enums::*;
use crate::resources::*;

use super::ui::InteractButton;
use super::ui::UiHandling;

#[derive(PartialEq)]
pub enum ButtonType {
    PlayButton,
    ShipBuilderButton,
    BackToMenuButton
}

#[derive(Resource)]
pub struct UIMenu {
    pub ui: Entity
}

#[derive(Resource)]
pub struct ShipBuilderPreview {
    pub ent: Entity,
    pub piece: char
}

#[derive(Component)]
pub struct MyButton {
    pub identifier: ButtonType
}

#[derive(Component)]
pub struct ShipBuilderButton {
    pub character: char,
}

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
            MyButton{ identifier: ButtonType::ShipBuilderButton })
        ).with_children(|button_parent| {
            button_parent.spawn(TextBundle::from_section("Ship Builder", TextStyle{
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
            ButtonType::ShipBuilderButton => {
                info!("Ship builder button pressed, going into.... something......");
                next_state.set(GameState::ShipBuilding);
            },
            ButtonType::BackToMenuButton => {
                info!("Back to Menu button pressed, going back.");
                next_state.set(GameState::MainMenu);
            }
        }
    }
}


// Ship builder
pub fn setup_ship_builder(
    mut commands: Commands,
    fonts: Res<FontResource>,
    colors: Res<Colors>,
    ship: Res<Ship>
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

    // Ship building buttons
    root_commands.with_children(|root_parent| {
        root_parent.spawn(
            NodeBundle{ 
                style: Style {
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
            // Spawn the ship building buttons
            root_parent.spawn(NodeBundle {
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
                // Spawn buttons for all characters
                for piece in SHIP_PIECES {
                    node_parent.spawn(
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
                        ShipBuilderButton{ character: piece },
                        InteractButton::from_clicked(colors.button_normal, colors.button_pressed)
                    )).with_children(|button_parent| {
                        button_parent.spawn(TextBundle{ text: Text::from_section(piece, fonts.p1_font.clone()), ..default() });
                    });
                }
            });
        });
    });

    let root_id = root_commands.id();
    commands.insert_resource(UIMenu{ ui: root_id });

    // Spawn the placement grid

    // TODO: that ^

    // Temporarily insert ship builder preview resource
    let preview_ent = commands.spawn(Text2dBundle{
        text: Text::from_section("$", fonts.preview_font.clone()),
        transform: Transform::from_translation(Vec3::new(-4000.0, 0.0, 2.0)).with_scale(Vec3::ONE * MOTHERSHIP_SCALE),
        ..default()
    }).id();

    commands.insert_resource(ShipBuilderPreview{ ent: preview_ent, piece: DOLLAR });


    // Add the ShipbuilderShip resource
    // Copy entities from the Ship resource
    let mut builder_ship = ShipbuilderShip::default();
    builder_ship.root = commands.spawn(SpatialBundle::default()).with_children(|root| {

        let left: i32 = -25;
        let bottom: i32 = -20;
        for x in 0..MOTHERSHIP_MAX_WIDTH {
            for y in 0..MOTHERSHIP_MAX_HEIGHT {
                if ship.pieces[x][y] == EMPTY_CHAR {
                    continue;
                }

                let pos = Vec3::new((x as i32 + left) as f32 * MOTHERSHIP_STRUCTURE_SPACING, (y as i32 + bottom) as f32 * MOTHERSHIP_STRUCTURE_SPACING, 0.0);
                info!("Spawning '{}' at {:?} grid: ({}, {})", ship.pieces[x][y], pos, x, y);

                let ent = root.spawn(
                    Text2dBundle{
                        text: Text::from_section(ship.pieces[x][y], fonts.p1_font.clone()),
                        transform: Transform::from_scale(Vec3::ONE * MOTHERSHIP_SCALE).with_translation(pos),
                        ..default()
                        }
                ).id();

                builder_ship.pieces[x][y] = Some(ent);
            }
        }
    }).id();

    commands.insert_resource(builder_ship);

}

pub fn exit_ship_builder(
    mut commands: Commands,
    builder_ship: Res<ShipbuilderShip>,
    preview: Res<ShipBuilderPreview>,
    menu: Res<UIMenu>
) {
    if let Some(root_entity) = commands.get_entity(menu.ui) {
        root_entity.despawn_recursive();
    }

    commands.remove_resource::<UIMenu>();

    if let Some(e_coms) = commands.get_entity(builder_ship.root) {
        e_coms.despawn_recursive();
    }
    commands.remove_resource::<ShipbuilderShip>();

    if let Some(e_coms) = commands.get_entity(preview.ent) {
        e_coms.despawn_recursive();
    }
    commands.remove_resource::<ShipBuilderPreview>();
}

pub fn ship_builder_navigation_buttons(
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

pub fn ship_builder_piece_buttons(
    mut preview: ResMut<ShipBuilderPreview>,
    button_query: Query<
        (
            &Interaction,
            &ShipBuilderButton
        ),
        (Changed<Interaction>, With<Button>)
    >,
    mut text_query: Query<&mut Text, (Without<Button>, Without<Interaction>, Without<ShipBuilderButton>)>
) {
    for (interaction, builder_button) in &button_query {
        match *interaction {
            Interaction::Clicked => {
                if let Ok(mut text) = text_query.get_mut(preview.ent) {
                    text.sections[0].value = String::from(builder_button.character);
                    preview.piece = builder_button.character;
                }
            },
            Interaction::Hovered => (),
            Interaction::None => ()
        }
    }
}

pub fn do_ship_builder_parts(
    mut commands: Commands,
    // For converting coordinates v
    window_query: Query<&Window, With<PrimaryWindow>>,
    cam_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // For getting and moving preview v
    preview_opt: Option<Res<ShipBuilderPreview>>,
    mut trans_query: Query<&mut Transform, (Without<Camera>, Without<Window>)>,
    // For destroying and placing pieces v
    input: Res<Input<MouseButton>>,
    fonts: Res<FontResource>,
    over_ui: Res<UiHandling>,
    mut ship: ResMut<Ship>,
    mut shipbuilder_ship: ResMut<ShipbuilderShip>,
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
    let grid_pos = Vec2::new(world_pos.x / MOTHERSHIP_STRUCTURE_SPACING, world_pos.y / MOTHERSHIP_STRUCTURE_SPACING).round();

    // Round to the nearest grid position
    let world_pos = Vec2::new(
        grid_pos.x * MOTHERSHIP_STRUCTURE_SPACING, 
        grid_pos.y * MOTHERSHIP_STRUCTURE_SPACING
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
        let x = (grid_pos.x as i32 - left).clamp(0, MOTHERSHIP_MAX_WIDTH as i32 - 1) as usize;
        let y = (grid_pos.y as i32 - bottom).clamp(0, MOTHERSHIP_MAX_HEIGHT as i32 - 1) as usize;

        if let Some(ent) = shipbuilder_ship.pieces[x][y] {
            if let Some(e_coms) = commands.get_entity(ent) {
                e_coms.despawn_recursive();
            }
            shipbuilder_ship.pieces[x][y] = None;
            info!("Deleting ship builder piece '{}' at: {:?}", ship.pieces[x][y], world_pos);
            ship.pieces[x][y] = '\0';
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

    if let Ok(mut trans) = trans_query.get_mut(preview.ent) {
        trans.translation = world_pos.extend(2.0);
    }
    // ^
    // Move Preview

    // Piece Placement v
    if input.pressed(place_key) {
        let left = -25;
        let bottom = -20;
        let x = (grid_pos.x as i32 - left).clamp(0, MOTHERSHIP_MAX_WIDTH as i32 - 1) as usize;
        let y = (grid_pos.y as i32 - bottom).clamp(0, MOTHERSHIP_MAX_HEIGHT as i32 - 1) as usize;

        if ship.pieces[x][y] != EMPTY_CHAR {
            return;
        }

        info!("Placing piece '{}' at {:?}", preview.piece, world_pos);
        let piece = commands.spawn(
            Text2dBundle
            { 
                transform: Transform::from_scale(Vec3::ONE * MOTHERSHIP_SCALE).with_translation(world_pos.extend(0.0)),
                text: Text::from_section(preview.piece, fonts.p1_font.clone()),
                ..default()
            }).id();
        
        // Make the spawned piece a child of the shipbuilder root so it'll be destroyed when destroying the root
        if let Some(mut e_coms) = commands.get_entity(shipbuilder_ship.root) {
            e_coms.add_child(piece);
        }

        shipbuilder_ship.pieces[x][y] = Some(piece);
        ship.pieces[x][y] = preview.piece;
    }
    // ^
    // Piece Placement
}
