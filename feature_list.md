# Feature List #

- 2 player deathmatch

- multiplayer, space sub builder, shoot each other to death

- some ascii characters are entities

    - no letters, numbers or control sequences

    - []\\|/() - armour

    - = torpedo launcher

    - ! - torpedo missiles

    - . underwater debris

    - \* could be explosion sparks?

    - @ reactor

    - {} reactor enclosure

    - propellers
        + -

    - o bubbles

- each character has a unique set of components

- create a sprite sheet for all the characters

- web assembly based game

- editor design stage: 

    - build a mothersub out of ascii characters

    - click and drag to draw lots of a type of character

    - name labels should display above a component button when you hover over it
 
- battle stage: 

    - press a button to deploy a fighter. this has a cooldown

    - click and hold in open space to shoot a laser

    - right click and aim once to shoot a torpedo

    - bubbles and debris created on impact that float upwards

    - impact points should break off and float. all components should do this

    - linear and angular velocity determined by number of fully powered thrusters

    - you cant see the other player so you have to use sonar to highlight them for a brief amount of time

## STRETCH GOALS ##

- type the text in the builder

- reactor meltdown/explosion

- reflect debris off mothersub hulls if they collide

- factor collision velocity between mothersubs into the integrity damage of Structure entities

- decrease integrity of torpedoes when hit by a bullet

- Thrust to Weight Ratio speed calculation

- energy system. you can only use the propellers and torpedo launchers that are fully powered by available generation.
