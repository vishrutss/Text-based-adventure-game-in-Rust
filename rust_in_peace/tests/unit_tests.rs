use game_lib::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_over() {
        // Create a world instance
        let mut world = World::new();

        // Scenario 1: Player's health is 0
        world.objects[LOC_PLAYER].health = Some(0);
        assert!(world.game_over());

        // Scenario 2: Player's health is not 0, but all enemies' health is 0
        world.objects[LOC_PLAYER].health = Some(100);
        world.objects[LOC_BEAR].health = Some(0);
        world.objects[LOC_TROLL].health = Some(0);
        world.objects[LOC_BANDITS].health = Some(0);
        assert!(world.game_over());

        // Scenario 3: Neither player's health is 0 nor all enemies' health is 0
        world.objects[LOC_PLAYER].health = Some(100);
        world.objects[LOC_BEAR].health = Some(100);
        world.objects[LOC_TROLL].health = Some(100);
        world.objects[LOC_BANDITS].health = Some(100);
        assert!(!world.game_over());
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

    #[test]
    fn test_do_consume() {
        let mut world = World::default();

        // Create the objects needed for the test
        let player_health = Some(80);

        // Set the initial world state
        world.objects[LOC_PLAYER].health = player_health;
        world.objects.push(Object {
            label: vec!["Apple".to_string()],
            description: "An apple (Get it to increase health)".to_string(),
            location: Some(LOC_TAVERN),
            destination: None,
            item: Some(true),
            enemy: false,
            health: Some(10),
            attack: None,
            consumable: Some(true),
        });

        // Test consuming an object
        let result = world.do_consume(Some(world.objects.len() - 1));

        assert_eq!(result, "You have consumed the item. Your health has increased to 90\n");
        assert_eq!(world.objects[LOC_PLAYER].health, Some(90));
        assert_eq!(world.objects[world.objects.len() - 1].location, None);
    }

    #[test]
    fn test_do_drop() {
        let mut world = World::default();

        // Set up the initial world state
        world.objects[LOC_PLAYER].location = Some(LOC_PLAYER);
        let object_index = world.objects.len();
        world.objects.push(Object {
            label: vec!["Sword".to_string()],
                    description: "A rusty sword.".to_string(),
                    location: Some(LOC_DUNGEONS),
                    destination: None,
                    item: Some(true),
                    enemy: false,
                    health: None,
                    attack: Some(20),
                    consumable: Some(false),
        });

        // Test dropping an object
        let result = world.do_drop(&"Sword".to_string());

        assert_eq!(result, "You are not holding any Sword.\n");
        assert_eq!(world.objects[object_index].location, Some(LOC_DUNGEONS));
    }


    #[test]
    fn test_player_here() {
        let mut world = World::default();

        // Set up the initial world state
        world.objects[LOC_PLAYER].location = Some(LOC_PLAYER);
        world.objects[LOC_FOREST].location = Some(LOC_FOREST);

        // Test when the player is at their location
        let result = world.player_here();
        assert_eq!(result, Some(LOC_PLAYER));

        // Test when the player is not at their location
        world.objects[LOC_PLAYER].location = Some(LOC_FOREST);
        let result = world.player_here();
        assert_eq!(result, Some(LOC_FOREST));
    }
}

