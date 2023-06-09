# Rust In Peace

## A text based adventure game

### By: Vishrut Sharma and Shrikrishna Bhat

### Sources

We have referred this tutorial to build the game, the link is provided below:
https://www.riskpeep.com/2022/08/make-text-adventure-game-rust-1.html
Some of the aspects of this game is derived from this tutorial.

# Overview

This project is one of the intersting projects in the field of Rust Programming, we have thorougly enjoyed developing this project and it utilizes the key aspects of rust which is performance and type safety.
This project was developed for the people who enjoy casual text-based games.
This text based game consists of a player wandering around the world fighting enemies. This game is a turn based game where choices are given to the player and player must select appropriate choice to move forward with the game. Initially the player will start at a location from where he can choose to go in any direction or any location which will be displayed by the 'map' command.
At the start of the game we will give a hint such that the player can choose help command to know what commands are available and hown the game can be continued from there.
For enemies we have currently added an enemy where the player have to defeat the enemy and the game will end. Player can also quit the game which will end the game.

Our game has following commands which we will explain briefly.

- **look:** This command allows the player to examine the surroundings and inspect specific objects or locations in the game.
  When the player enter look command it lists the objects in the surrounding and player is transported to that location.
- **go:** This command lets the player go to a specified location, the game state gets updated when the player enters a specific location.
- **attack:** This command mainly lets the player attack an enemy. We have introduced a health system for both players and enemies, the health get decreased by random and player also has the choice to run away from the enemy.
- **get:** Next set of commands are the inventory commands which are crucial for health system and to end the game. Get command is used to pick up the object like health like an apple or weapon like a bow.
- **drop:** This command is used to drom the inventory items which are not needed.
- **inventory:** This command is used to check the inventory items to view the current inventory status.
- **map:** This command is used to view the current locations which are defined in the game.
- **quit:** This command is used to quit the game.

## Building and running the project

Building and running our project is pretty much simple, just we have to use cargo run and you will get the option to start the game.<br>
You can use \<help\> to understand what are the commands available. If the game isnt completed you can quit the game using \<quit\> command.<br>
We have used 5 dependencies for our code namely:

- clearscreen: To flush the screen
- regex: The regular expression dependency, such that it will work for both upper and lowercases.
- serde: We are taking the object location details from the file, hence we will deserialize the data stored in the file.
- ron: This is rust object notation dependency mainly used for saving out file which is in the ron format.
- rand: This is for random values.

## Testing

We have manually tested the code to check what features were working and what were not. One such thing is the random attack where game didnt end and the value was going less that 0, because we had not set the type properly, Hence, minor errors like these we fixed by running the code multiple times.
We gave this game to our friend to try and he was not given any information except to use the cargo run command. He worked his way through the enemy and defeated the enemy and game ended. We also have tested some exception where attack was not taking place properly and fixed it.

## Example of our code

[![Screenshot-2023-06-08-at-7-34-15-PM.png](https://i.postimg.cc/Gtgf3mhY/Screenshot-2023-06-08-at-7-34-15-PM.png)](https://postimg.cc/YLgRbkFq)

## TLDR

## License

We have added the MIT License in the repository.
