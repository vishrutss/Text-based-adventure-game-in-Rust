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
        assert_eq!(game.game_over(), true);

        // Scenario 2: Player's health is not 0, but all enemies' health is 0
        game.objects[LOC_PLAYER].health = Some(100);
        game.objects[LOC_BEAR].health = Some(0);
        game.objects[LOC_TROLL].health = Some(0);
        game.objects[LOC_BANDITS].health = Some(0);
        assert_eq!(game.game_over(), true);

        // Scenario 3: Neither player's health is 0 nor all enemies' health is 0
        game.objects[LOC_PLAYER].health = Some(100);
        game.objects[LOC_BEAR].health = Some(100);
        game.objects[LOC_TROLL].health = Some(100);
        game.objects[LOC_BANDITS].health = Some(100);
        assert_eq!(game.game_over(), false);
    }
}
