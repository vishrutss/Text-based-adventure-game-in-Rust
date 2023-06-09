# Rust In Peace

## A text based adventure game

### By: Vishrut Sharma and Shrikrishna Bhat

### Sources

We have referred this tutorial to build the game, the link is provided below:
https://www.riskpeep.com/2022/08/make-text-adventure-game-rust-1.html.
Some of the aspects of the game are derived from this tutorial.

# Overview

This project is one of the interesting projects in the field of Rust Programming. We have thoroughly enjoyed developing this project, which utilizes the key aspects of Rust: performance and type safety.

The project was created for people who enjoy casual text-based games. This text-based game involves a player wandering around the world and fighting enemies. It is a turn-based game where the player is presented with choices to progress. Initially, the player starts at a specific location and can choose to go in any direction or visit different locations, which can be displayed using the 'map' command.

At the beginning of the game, we provide a hint to help the player understand the available commands and how to proceed. The player must defeat all enemies in the game world to win the game. If the player dies the game ends immediately. Additionally, the player has the option to quit the game, which will also end it.

Our game has following commands which we will explain briefly.

- **look:** This command allows the player to examine the surroundings and inspect specific objects or locations in the game.
  This command lists the objects in the immediate vicinity. Including weapons, healing items, paths, and location descriptions.
- **go:** This command lets the player go to a specified location, the game state gets updated when the player enters a specific location. The player can only go to a certain location if there is a path to the desired destination. If there is no path to the desired destination the player will not be able to go to that location and will have to find another way.
- **attack:** This command mainly lets the player attack an enemy. This command enters the player into an attack state/mode. The only commands that work while in this state are ```use <weapon name>```, ```inventory```, and ```run```. The "use" command will perform the attack, the "inventory" command will display the weapons that the player has, and the "run" command will exit the player from the attack state/mode. We have introduced a health system for both players and enemies. The damage done by the player is fixed depending on the weapon used, and the damage done by the enemy is random. If the player chooses to run from the fight the health of the enemy is regenerated to 100 but the player will not regenerate and will have to consume healing items.
- **get:** This command is used to pick up objects like healing items or a weapon.
- **drop:** This command is used to drop the items from the inventory which are not needed.
- **inventory:** This command is used to check the inventory items and view the current inventory status.
- **map:** This command is used to view the locations which are defined in the game.
- **quit:** This command is used to quit the game.

## Building and running the project

Building and running our project is pretty simple, we just have to use ```cargo run``` and you will get the option to start the game.<br>
You can use \<help\> to understand what commands are available. If the game isn't completed you can quit the game using \<quit\> command.<br>
We have used 5 dependencies for our code namely:

- clearscreen: To flush the screen
- regex: The regular expression dependency, such that it will work for both upper and lowercases.
- serde: We are taking the object location details from the file ```game_file.ron```, hence we will deserialize the data stored in the file.
- ron: This is a rust object notation dependency mainly used for saving out file which is in the ron format.
- rand: This is for random values.

## Testing

We have manually tested the code to check what features were working and what were not. Some of the problems we faced was while handling unexpected inputs from the player. One such problem we faced was when the player writes  ```get forest``` which is an invalid input the "forest" was getting added to the inventory. Hence, minor errors like these were fixed by running the code multiple times.<br>
Another way we tested the game was by having our friend try the game. He was not given any prior information about the game and had no knowledge of how the game functioned. He worked his way through the game and defeated an enemy and helped us find a bug where we had forgotten to code the scenario of when the enemies health had been depleted to 0. The game didn't end and the player could keep attacking the enemy and the game would crash becase the enemies health would go into the negatives. We eventually fixed the bug. We have also tested some exceptions where the attack was not taking place properly and fixed it.

## Example of our code

[![Screenshot-2023-06-08-at-7-34-15-PM.png](https://i.postimg.cc/Gtgf3mhY/Screenshot-2023-06-08-at-7-34-15-PM.png)](https://postimg.cc/YLgRbkFq)

## TODO

## License

We have added the MIT License in the repository.
