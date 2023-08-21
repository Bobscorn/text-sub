use bevy::prelude::*;
use crate::events::*;
use crate::enums::*;
use crate::resources::*;

use super::ui::InteractButton;

#[derive(PartialEq)]
pub enum ButtonType {
    PlayButton,
    ShipBuilderButton,
}

#[derive(Resource)]
pub struct UIMenu {
    pub ui: Entity
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

pub fn handle_main_menu_buttons(
    interaction_query: Query<
        (
            &Interaction,
            &MyButton
        ),
        (Changed<Interaction>, With<Button>)
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
    let root = commands.spawn(
        NodeBundle{ 
            style: Style {
                size: Size { width: Val::Percent(100.0), height: Val::Percent(100.0) },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::End,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            } ,
            background_color: colors.menu_background.into(),
            ..default()
        }
    ).with_children(|root_parent| {
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
            node_parent.spawn((ButtonBundle {
                style: Style {
                    size: Size { width: Val::Px(40.0), height: Val::Percent(40.0) },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: colors.button_normal.into(),
                ..default()
            }, InteractButton::from_clicked(colors.button_normal, colors.button_pressed)));
        });
    }).id();

    commands.insert_resource(UIMenu{ ui: root });
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
