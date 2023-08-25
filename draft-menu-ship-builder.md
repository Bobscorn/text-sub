DRAFT FOR sub BUILDER AND MAIN MENU:

 - New Game States
 - UI For Main Menu
    - Build subs button -> go to builder scene
    - Play Button -> Go to MatchMaking state
    - Draft other buttons:
        - Quit to main menu (exit session)
        - Options (possibly a stretch goal)
 - UI For sub Builder
    - Draft: 
        - Buttons to pick a sub part to place
        - horizontal tab displays categories of sub parts
        - Visible Grid to place sub parts onto
        - Left click (or hold) to place picked part
        - Right click (or hold) to erase part
        - type to enter a valid character in the right-adjacent slot
        - E/Q/Scroll wheel (without shift) to rotate hovered piece
        - Middle Mouse + Mouse Movement/WASD/Arrow keys to move around Grid
        - Shift + Scroll wheel/Page Up/Page Down to zoom in or out

Update 0.1:
~~- Getting the sub builder and in match versions of subs to look the same is proving difficult,~~
~~- The scales are for some reason not matching up~~
Update 0.2:
- How to send/store sub data?
- Initially a resource made the most sense
    - Can't find how to synchronise resources across peers via matchbox/bevy_ggrs
    - Maybe try send socket-level message?
- Remembering that any entity with a rollback component gets synchronised there's also the possibility to create an 'update sub' entity
    - This can update players' resources that contain the sub data and a system can consume the 'update sub' entity and update both the visual and resource data
Update 0.3:
- Turns out the scales in the multiplayer.rs file were not the way they were expected
- it was applying the translation and then the scale, which meant the scale factor was affecting the translation.
- Fixed by doing 
    ```rust
    Transform::from_scale(Vec3::ONE * MOTHERsub_SCALE).with_translation(pos)
    ```
    instead of 
    ```rust
    Transform::from_scale(Vec3::ONE * MOTHERsub_SCALE) * Transform::from_translation(pos)
    ```
