# Feature List #

- 2 player deathmatch

- multiplayer, space ship builder, shoot each other to death

- some ascii characters are entities

    - no letters, numbers or control sequences

    - [] \\|/ - light armor

    - = torpedo launcher (turrets, launchers can be rotated)

    - % - fighter ship/drone (shoots mini-torpedo)

    - ` ~ ! - torpedo missiles

    - . stars

    - \* could be explosion sparks?

    - () shield entities

    - \+ (plus), # (hash) heavy armor

    - @ reactor

    - {} reactor enclosure

    - $ thruster exhaust?

    - ? minigun turret

    - ' bullet

    - ; ????

    - : hangar shield door?

    - <> thrusters

- each character has a unique set of components

- create a sprite sheet for all the characters

- web assembly based game

- design stage: 

    - build a mothership out of ascii characters

    - click and drag to draw lots of a type of character
 
- battle stage: 

    - press a button to deploy a fighter. this has a cooldown

    - click and hold in open space to shoot a laser

    - right click and aim once to shoot a torpedo

## STRETCH GOALS ##

- reactor meltdown/explosion

- player controlled velocity

- reflect debris off mothership hulls if they collide

- factor collision velocity between motherships into the integrity damage of Structure entities

- decrease integrity of torpedoes when hit by a bullet
