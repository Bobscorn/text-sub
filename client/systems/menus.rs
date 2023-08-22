use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::MainCamera;
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
    pub ent: Entity
}

#[derive(Component)]
pub struct MyButton {
    pub identifier: ButtonType
}

#[derive(Component)]
pub struct ShipBuilderButton {
    pub character: String,
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
    colors: Res<Colors>
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
                    size: Size { width: Val::Percent(100.0), height: Val::Percent(30.0) },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: colors.node_background.into(),
                ..default()
            }).with_children(|node_parent| {
                // for now just a simple button
                node_parent.spawn((ButtonBundle {
                    style: Style {
                        size: Size { width: Val::Px(40.0), height: Val::Px(40.0) },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: colors.button_normal.into(),
                    ..default()
                }, InteractButton::from_clicked(colors.button_normal, colors.button_pressed)));
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
        ..default()
     }).id();

     commands.insert_resource(ShipBuilderPreview{ ent: preview_ent });
}

pub fn exit_ship_builder(
    mut commands: Commands,
    menu: Res<UIMenu>
) {
    if let Some(root_entity) = commands.get_entity(menu.ui) {
        root_entity.despawn_recursive();
    }

    commands.remove_resource::<UIMenu>();
}

pub fn handle_ship_builder_buttons(
    colors: Res<Colors>,
    mut interaction_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &MyButton
            ),
            (Changed<Interaction>, With<Button>)
        >,
    mut next_state: ResMut<NextState<GameState>>
) {

}

pub fn do_ship_builder_parts(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    cam_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    input: Res<Input<MouseButton>>,
    over_ui: Res<UiHandling>,
    fonts: Res<FontResource>,
    preview_opt: Option<Res<ShipBuilderPreview>>,
    mut trans_query: Query<&mut Transform, (Without<Camera>, Without<Window>)>
) {
    if over_ui.is_pointer_over_ui {
        return;
    }

    // Do all that juicy cursor pos -> world pos stuff
    // Possibly unnecessary safety, there's not a real reason there won't be a main camera or primary window

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

    // Use the camera's raycast method to get a world space pos (the origin of the ray)
    let ray = match cam.viewport_to_world(cam_trans, cursor_pos) {
        Some(r) => r,
        None => return
    };

    // *wipes forehead* phew that took a lot of matching
    let world_pos = ray.origin.truncate();

    // Move preview if it exists
    if let Some(preview) = preview_opt {
        if let Ok(mut trans) = trans_query.get_mut(preview.ent) {
            trans.translation = world_pos.extend(0.0);
        }
    }

    if input.just_pressed(MouseButton::Left) {
        info!("Just clicked on {:?}!", world_pos);
    
        commands.spawn(
            Text2dBundle
            { 
                transform: Transform::from_translation(world_pos.extend(0.0)),
                text: Text::from_section("$", fonts.p1_font.clone()),
                ..default()
            });
    }
}
