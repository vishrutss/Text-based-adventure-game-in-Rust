# Rust In Peace

## A Text Based Adventure Game

### Submitted By: Vishrut Sharma and Shrikrishna Bhat

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
- **go:** This command lets the player go to a specified location, the game state gets updated when the player enters a specific location. The player can either mention the location name or direction to go to a location. For example a player can either type `go north` or `go tavern` Available directions are North, East, West, South. The player can only go to a certain location if there is a path to the desired destination. If there is no path to the desired destination the player will not be able to go to that location and will have to find another way.
- **attack:** This command mainly lets the player attack an enemy. This command enters the player into an attack state/mode. The only commands that work while in this state are `use <weapon name>`, `inventory`, and `run`. The `use` command will perform the attack, the `inventory` command will display the weapons that the player has, and the `run` command will exit the player from the attack state/mode. We have introduced a health system for both players and enemies. The damage done by the player is fixed depending on the weapon used, and the damage done by the enemy is random. If the player chooses to run from the fight the health of the enemy is regenerated to 100 but the player will not regenerate and will have to consume healing items.
- **get:** This command is used to pick up objects like healing items or a weapon.
- **drop:** This command is used to drop the items from the inventory which are not needed.
- **inventory:** This command is used to check the inventory items and view the current inventory status.
- **map:** This command is used to view the locations which are defined in the game.
- **quit:** This command is used to quit the game.

## Methodology

Our game consists of 3 main things:<br>

- **Command** - This is an enumeration which contains all the commands like drop, get, attack, look, go, etc.
- **Object** - This is a structure type which contains label, location, item, enemy. Basically all the objects which are present in the game.
- **World** - This is also a structure type which contains the object structure which are stored as a vector of objects.

  When we implement the World structure, we define the various locations and other functions, such as how the game should end, how it should proceed, and how the attack on the enemy should be executed. Each function is implemented within the world structure.

  Various location objects are stored in the RON file. Initially, we serialize the data and store it in the file. Later, we deserialize it and use it in our game.

  Another aspect of the game is the **health** variable, which we have assigned to both the enemy and the player. The amount of health taken by an enemy attack is random, and the health taken from the enemy by the player is fixed depending on the weapon used. For example, a sword will do 20 damage and a bow will do 15 damage to an enemy. There is also a chance for an enemy attack to miss, which is determined randomly

  We also have an inventory system which displays a list of weapons and health items.

  We have utilized an iterative approach to program this project, where we developed each component iteratively and linked them together.

## Building and running the project

Building and running our project is pretty simple, we just have to use `cargo run` and you will get the option to start the game.

You can use \<help\> to understand what commands are available. If the game isn't completed you can quit the game using \<quit\> command.

We have used 5 dependencies for our code namely:

- clearscreen: To flush the screen
- regex: The regular expression dependency, such that it will work for both upper and lowercases.
- serde: We are taking the object location details from the file `game_file.ron`, hence we will deserialize the data stored in the file.
- ron: This is a rust object notation dependency mainly used for saving out file which is in the ron format.
- rand: This is for random values.

## Testing

We conducted manual testing of the code to identify working and non-working features. One challenge we encountered was handling unexpected inputs from the player. For example, when the player entered an invalid input like `get forest`, the object `forest` would mistakenly get added to the inventory. To address such minor errors, we ran the code multiple times and made necessary fixes.

Another method of testing the game involved having a friend play it without any prior knowledge or understanding of how it functioned. As he progressed through the game, he encountered and defeated an enemy, which helped us discover a bug. We had forgotten to account for the scenario when the enemy's health was depleted to 0. The game didn't end, allowing the player to keep attacking the enemy until the game crashed due to negative enemy health. We eventually fixed this bug.

Additionally, we tested for exceptions where the attack was not functioning correctly and resolved those issues as well.

We also wrote some unit tests for some of the function and we have created a folder called tests and added a file `unit_tests.rs`. Some of the functions for which tests are written are - game_over(), do_consume(), do_look(), etc.

## Link to Video

[Click to see Video](https://gitlab.cecs.pdx.edu/a-text-based-adventure-game/rust-in-peace/-/blob/main/PRESENTATION.mp4)

## Example of our code

[![Screenshot-2023-06-08-at-7-34-15-PM.png](https://i.postimg.cc/Gtgf3mhY/Screenshot-2023-06-08-at-7-34-15-PM.png)](https://postimg.cc/YLgRbkFq)

## Expectations

Most of the features we developed met our expectations. However, the `Attack` functionality initially did not work as intended. We addressed this issue and ensured that it functioned properly. Our goal was to create a game that is easy to play with simple instructions and enjoyable for players. To ensure this, we gave the game to one of our friends who had no knowledge of Rust or the game we developed. Their valuable feedback helped us improve the game and debug any errors.

Initially, we planned to include NPCs (non-player characters) in the game, but due to time constraints, we were unable to add them.

In the future, we envision expanding the game by adding more locations, additional enemies, a story, and even introducing an NPC who could become a partner within the game.

## License

We have added the MIT License in the repository.
