DRAFT FOR SHIP BUILDER AND MAIN MENU:

 - New Game States
 - UI For Main Menu
    - Build Ships button -> go to builder scene
    - Play Button -> Go to MatchMaking state
    - Draft other buttons:
        - Quit to main menu (exit session)
        - Options (possibly a stretch goal)
 - UI For Ship Builder
    - Draft: 
        - Buttons to pick a ship part to place
        - horizontal tab displays categories of ship parts
        - Visible Grid to place ship parts onto
        - Left click (or hold) to place picked part
        - Right click (or hold) to erase part
        - type to enter a valid character in the right-adjacent slot
        - E/Q/Scroll wheel (without shift) to rotate hovered piece
        - Middle Mouse + Mouse Movement/WASD/Arrow keys to move around Grid
        - Shift + Scroll wheel/Page Up/Page Down to zoom in or out

Update 0.1:
- Getting the ship builder and in match versions of ships to look the same is proving difficult,
- The scales are for some reason not matching up
Update 0.2:
- How to send/store ship data?
- Initially a resource made the most sense
    - Can't find how to synchronise resources across peers via matchbox/bevy_ggrs
    - Maybe try send socket-level message?
- Remembering that any entity with a rollback component gets synchronised there's also the possibility to create an 'update ship' entity
    - This can update players' resources that contain the ship data and a system can consume the 'update ship' entity and update both the visual and resource data
