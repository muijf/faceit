use faceit::HttpClient;

#[tokio::main]
async fn main() -> Result<(), faceit::error::Error> {
    let client = HttpClient::new();

    // Example 1: Player API
    println!("=== Player API ===");
    let player = faceit::http::ergonomic::Player::new("player-id-here", &client);

    // Get player details
    match player.get().await {
        Ok(player_data) => {
            println!("Player: {}", player_data.nickname);
            if let Some(country) = player_data.country {
                println!("Country: {}", country);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Get player stats
    match player.stats("cs2").await {
        Ok(stats) => {
            println!("Stats retrieved for game: {}", stats.game_id);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Get player match history
    match player.history("cs2", None, None, Some(0), Some(20)).await {
        Ok(history) => {
            println!("Found {} matches", history.items.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Get player hubs
    match player.hubs(Some(0), Some(50)).await {
        Ok(hubs) => {
            println!("Found {} hubs", hubs.items.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 2: Match API
    println!("\n=== Match API ===");
    let match_obj = faceit::http::ergonomic::Match::new("match-id-here", &client);

    match match_obj.get().await {
        Ok(match_data) => {
            println!("Match ID: {}", match_data.match_id);
            println!("Status: {}", match_data.status);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    match match_obj.stats().await {
        Ok(stats) => {
            println!("Rounds: {}", stats.rounds.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 3: Game API
    println!("\n=== Game API ===");
    let game = faceit::http::ergonomic::Game::new("cs2", &client);

    match game.get().await {
        Ok(game_data) => {
            println!("Game: {}", game_data.long_label);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    match game.matchmakings(Some("EU"), Some(0), Some(20)).await {
        Ok(matchmakings) => {
            println!("Found {} matchmakings", matchmakings.items.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 4: Hub API
    println!("\n=== Hub API ===");
    let hub = faceit::http::ergonomic::Hub::new("hub-id-here", &client);

    match hub.get(None).await {
        Ok(hub_data) => {
            println!("Hub: {}", hub_data.name);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    match hub.matches(Some("all"), Some(0), Some(20)).await {
        Ok(matches) => {
            println!("Found {} matches", matches.items.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    match hub.members(Some(0), Some(50)).await {
        Ok(members) => {
            println!("Found {} members", members.items.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 5: Championship API
    println!("\n=== Championship API ===");
    let championship = faceit::http::ergonomic::Championship::new("championship-id-here", &client);

    match championship.get(None).await {
        Ok(championship_data) => {
            println!("Championship: {}", championship_data.name);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    match championship.matches(Some("all"), Some(0), Some(20)).await {
        Ok(matches) => {
            println!("Found {} matches", matches.items.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 6: Direct instantiation (alternative to client methods)
    println!("\n=== Direct Instantiation ===");
    use faceit::http::ergonomic::{Championship, Game, Hub, Match, Player};

    let _player2 = Player::new("player-id-here", &client);
    let _match_obj2 = Match::new("match-id-here", &client);
    let _game2 = Game::new("cs2", &client);
    let _hub2 = Hub::new("hub-id-here", &client);
    let _championship2 = Championship::new("championship-id-here", &client);

    println!("Created ergonomic API instances directly");

    Ok(())
}
