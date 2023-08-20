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
        .any(|i| matches!(i, Interaction::Clicked | Interaction::Hovered));
}
