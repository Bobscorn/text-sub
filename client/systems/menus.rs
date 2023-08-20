use bevy::prelude::*;
use crate::events::*;
use crate::enums::*;

#[derive(PartialEq)]
pub enum ButtonType {
    PlayButton,
    // QuitButton
}

#[derive(Resource)]
pub struct MainMenu {
    pub ui: Entity
}

#[derive(Component)]
pub struct MyButton {
    pub identifier: ButtonType
}

#[derive(Resource)]
pub struct MenuColors {
    menu_background: Color,
    button_normal: Color,
    button_pressed: Color,
    button_selected: Color
}

pub fn setup_mainmenu(
    mut commands: Commands,
    fonts: Res<FontResource>
) {
    let colors = MenuColors{ menu_background: Color::rgb(0.1, 0.1, 0.1), button_normal: Color::GRAY, button_pressed: Color::DARK_GRAY, button_selected: Color::rgb(0.35, 0.35, 0.35) };

    let root = commands.spawn(NodeBundle{
        style: Style {
            size: Size { width: Val::Percent(100.0), height: Val::Percent(100.0) },
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn((ButtonBundle {
            style: Style {
                size: Size { width: Val::Px(150.0), height: Val::Px(50.0) },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: colors.menu_background.into(),
            ..default()
        }, MyButton{ identifier: ButtonType::PlayButton }))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Play", TextStyle{ 
                font: fonts.font.clone(),
                font_size: 40.0,
                color: Color::rgb(0.8, 0.8, 0.8)
            }));
        });
    }).id();

    commands.insert_resource(MainMenu{ ui: root });
    commands.insert_resource(colors);
}

pub fn exit_menu(
    mut commands: Commands,
    menu_res: Res<MainMenu>
) {
    if let Some(ent) = commands.get_entity(menu_res.ui) {
        ent.despawn_recursive();
    }
    commands.remove_resource::<MainMenu>();
}

pub fn handle_buttons(
    colors: Res<MenuColors>,
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

        if button_type.identifier == ButtonType::PlayButton && *interaction == Interaction::Clicked {
            info!("Play button pressed, going into matchmaking.");
            next_state.set(GameState::MatchMaking);
        }
    }
}
