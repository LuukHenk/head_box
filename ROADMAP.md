# Bugs

# User stories v0.0.1

### Add sound effects
- Taking damage sound (player and enemies)
- Dying sound (player and enemies)

### Add proper sprite movement
- Hurt animation when a player or zombie is hurt

### Update weapons
- Make the shotgun bullets shorter
- Add a different bullet texture per weapon
- Add a weapon for the zombie (instead of just touching you)


# User stories for more and better enemies (v0.0.2?)

### Add demon
- Shoots fireballs
- quite strong, but slow

### Add tiny zombie
- Fast
- Not so strong

### Add more life to the enemies and player
- Blood spawns when damaged
- Zombies stay on the ground when dying (should de-spawn after a minute or so)







# User stories for main menu

### Styling
- The main menu needs some styling









# User stories for the inventory and score (v0.0.3?)

### Add inventory bar
- The spritesheet has a nice inventory bar which shows the active weapon, ammo and the gun;
- However, this would require an extra camera in Bevy which currently seems broken... So lets wait.

## Add Life indicator
- Add life indicator in the inventory bar

## Add Ammo
- Add ammo indicator in the inventory bar
- Add ammo pickups?

### Add Score
- Each time a zombie is killed, you get some points
- Show the points on the inventory bar

### Final score indicator
- Add final score screen with exit and restart button

### Add score multiplier
- The faster you kill enemies, the higher the multiplier
- Display the multiplier on the inventory bar






# User stories for the map (v0.1.0?)

### Add collision to the objects on the map
- The newly created map does not have any collision yet, add collision

### Draw the other half of the arena
- Draw the other half of the arena using the SVG file in src/assets/textures/arena
- Create a new ticket for adding collision to the components on this part of the map, and for removing the walls on the first part of the map

### Enemy pathfinding
- When there is more collisions on the map, the enemies will get stuck. We need some way to prevent this








# User stories for more menus

### Pause menu
- We need a pause menu over the game

### Settings menu
- We need a settings menu, which can be accessed via the pause menu and the main menu

### Volume options
- Add volume options to the settings menu 
