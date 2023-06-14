use game_lib::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_over() {
        // Create a game instance
        let mut game = World::new();

        // Scenario 1: Player's health is 0
        game.objects[LOC_PLAYER].health = Some(0);
        assert!(game.game_over());

        // Scenario 2: Player's health is not 0, but all enemies' health is 0
        game.objects[LOC_PLAYER].health = Some(100);
        game.objects[LOC_BEAR].health = Some(0);
        game.objects[LOC_TROLL].health = Some(0);
        game.objects[LOC_BANDITS].health = Some(0);
        assert!(game.game_over());

        // Scenario 3: Neither player's health is 0 nor all enemies' health is 0
        game.objects[LOC_PLAYER].health = Some(100);
        game.objects[LOC_BEAR].health = Some(100);
        game.objects[LOC_TROLL].health = Some(100);
        game.objects[LOC_BANDITS].health = Some(100);
        assert!(!game.game_over());
    }

    #[test]
fn test_update_state() {
    let mut world = World::new();
    // Test case 1: Quit command
    let command = Command::Quit;
    let result = world.update_state(&command);
    assert_eq!(result, "Quitting.\nThank you for playing!"); // Checking if the result matches the expected message

    // Test case 2: Unknown command
    let command = Command::Unknown("InvalidCommand".to_string());
    let result = world.update_state(&command);
    assert!(result.contains("Invalid command!!")); // Checking if the result contains the expected message

}

#[test]
fn test_do_look() {
    let mut world = World::new();

    // Set up the objects and player's location
    world.objects[LOC_PLAYER].location = Some(LOC_FOREST);
    world.objects[LOC_FOREST].label = vec!["Forest".to_string()];
    world.objects[LOC_FOREST].description = "Look out for tree people.".to_string();

    // Test case 1: Look without specifying a noun
    let result = world.do_look("");
    let expected = " You are in the Forest\n Look out for tree people..\n\nYou see:\nA path to the north leading out of the forest leading to an old Tavern\n";
    assert_eq!(result, expected);

    // Test case 2: Look with an invalid noun
    let result = world.do_look("invalid");
    let expected = "Invalid command!!\n";
    assert_eq!(result, expected);
}

    #[test]
    fn test_do_go() {
        let mut world = World::new();
        let result = world.do_go(&"South".to_string());
        assert_eq!(result, "You see nothing but trees. There is no other path in that direction.");

        let result = world.do_go(&"Invalid".to_string());
        assert_eq!(result, "Invalid command!!");
    }
}

