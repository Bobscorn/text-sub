use bevy::prelude::*;
use crate::events::*;
use crate::enums::*;
use crate::resources::*;

#[derive(PartialEq)]
pub enum ButtonType {
    PlayButton,
    ShipBuilderButton,
}

#[derive(Resource)]
pub struct MainMenu {
    pub ui: Entity
}

#[derive(Component)]
pub struct MyButton {
    pub identifier: ButtonType
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
        }, MyButton{ identifier: ButtonType::PlayButton }))
        .with_children(|button_parent| {
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
        }, MyButton{ identifier: ButtonType::ShipBuilderButton }))
        .with_children(|button_parent| {
            button_parent.spawn(TextBundle::from_section("Ship Builder", TextStyle{
                font: fonts.font.clone(),
                font_size: 40.0,
                color: Color::rgb(0.8, 0.8, 0.8)
            }));
        });
    }).id();

    commands.insert_resource(MainMenu{ ui: root });
}

pub fn exit_main_menu(
    mut commands: Commands,
    menu_res: Res<MainMenu>
) {
    if let Some(ent) = commands.get_entity(menu_res.ui) {
        ent.despawn_recursive();
    }
    commands.remove_resource::<MainMenu>();
}

pub fn handle_buttons(
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
    for (interaction, mut background, button_type) in &mut interaction_query {
        
        match *interaction {
            Interaction::Clicked => {
                *background = colors.button_pressed.into();
            },
            Interaction::None => {
                *background = colors.button_normal.into();
            },
            Interaction::Hovered => (),
        }

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
