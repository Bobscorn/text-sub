use bevy::prelude::*;

// This code and strategy was taken from a code snippet by https://github.com/hxYuki in this issue for the bevy engine https://github.com/bevyengine/bevy/issues/3570

// Check this resource to know if the pointer is over UI or not
#[derive(Resource, Default)]
pub struct UiHandling {
    pub is_pointer_over_ui: bool,
}

// Add this component to any UI that shouldn't be considered (for example UI you can't click)
#[derive(Component)]
pub struct NoPointerCapture;

// Add to any button that should change color upon clicking or hovering
#[derive(Component)]
pub struct InteractButton {
    pub normal_color: Color,
    pub clicked_color: Color,
    pub hovered_color: Color
}

impl InteractButton {
    pub fn from_colors(normal_color: Color, clicked_color: Color, hovered_color: Color) -> InteractButton {
        InteractButton { normal_color: normal_color, clicked_color: clicked_color, hovered_color: hovered_color }
    }

    pub fn from_clicked(normal_color: Color, clicked_color: Color) -> InteractButton {
        InteractButton { normal_color: normal_color, clicked_color: clicked_color, hovered_color: normal_color }
    }

    pub fn from_normal(normal_color: Color) -> InteractButton {
        InteractButton { normal_color: normal_color, clicked_color: normal_color, hovered_color: normal_color }
    }
}

// This system populates the UiHandling resource's value
// Make sure to have this system run before any systems reading from UiHandling
pub fn check_ui_interaction(
    mut ui_handling: ResMut<UiHandling>,
    interaction_query: Query<
        &Interaction,
        (With<Node>, Changed<Interaction>, Without<NoPointerCapture>),
    >,
) {
    ui_handling.is_pointer_over_ui = interaction_query
        .iter()
        .any(|i| matches!(i, Interaction::Pressed | Interaction::Hovered));
}

pub fn handle_interaction_buttons(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &InteractButton
    ), (
        Changed<Interaction>, With<Button>
    )>
) {
    for (interaction, mut back_color, butt) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *back_color = butt.clicked_color.into();
            },
            Interaction::Hovered => {
                *back_color = butt.hovered_color.into();
            },
            Interaction::None => {
                *back_color = butt.normal_color.into();
            }
        }
    }
}
