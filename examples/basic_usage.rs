use faceit::HttpClient;

#[tokio::main]
async fn main() -> Result<(), faceit::error::Error> {
    // Create a client (without authentication)
    let client = HttpClient::new();

    // Or create a client with API key or access token:
    // let client = Client::builder()
    //     .api_key("your-api-key-or-access-token")
    //     .build()?;

    // Example 1: Get player by ID
    println!("Fetching player by ID...");
    match client.get_player("player-id-here").await {
        Ok(player) => {
            println!("Player: {}", player.nickname);
            if let Some(country) = player.country {
                println!("Country: {}", country);
            }
        }
        Err(e) => eprintln!("Error fetching player: {}", e),
    }

    // Example 2: Get player from lookup by nickname
    println!("\nFetching player from lookup...");
    match client
        .get_player_from_lookup(Some("player_nickname"), Some("cs2"), None)
        .await
    {
        Ok(player) => {
            println!("Player: {}", player.nickname);
        }
        Err(e) => eprintln!("Error fetching player: {}", e),
    }

    // Example 3: Get player stats
    println!("\nFetching player stats...");
    match client.get_player_stats("player-id-here", "cs2").await {
        Ok(stats) => {
            println!("Player ID: {}", stats.player_id);
            println!("Game ID: {}", stats.game_id);
        }
        Err(e) => eprintln!("Error fetching stats: {}", e),
    }

    // Example 4: Get player match history
    println!("\nFetching player match history...");
    match client
        .get_player_history("player-id-here", "cs2", None, None, Some(0), Some(20))
        .await
    {
        Ok(history) => {
            println!("Found {} matches", history.items.len());
            if let Some(first_match) = history.items.first() {
                println!("Most recent match: {}", first_match.match_id);
            }
        }
        Err(e) => eprintln!("Error fetching history: {}", e),
    }

    // Example 5: Get match details
    println!("\nFetching match details...");
    match client.get_match("match-id-here").await {
        Ok(match_details) => {
            println!("Match ID: {}", match_details.match_id);
            println!("Game: {}", match_details.game);
            println!("Status: {}", match_details.status);
        }
        Err(e) => eprintln!("Error fetching match: {}", e),
    }

    // Example 6: Get match stats
    println!("\nFetching match stats...");
    match client.get_match_stats("match-id-here").await {
        Ok(stats) => {
            println!("Rounds: {}", stats.rounds.len());
        }
        Err(e) => eprintln!("Error fetching match stats: {}", e),
    }

    // Example 7: Search for players
    println!("\nSearching for players...");
    match client
        .search_players("player_name", Some("cs2"), None, Some(0), Some(20))
        .await
    {
        Ok(results) => {
            println!("Found {} players", results.items.len());
            for player in results.items.iter().take(5) {
                println!("  - {} ({})", player.nickname, player.player_id);
            }
        }
        Err(e) => eprintln!("Error searching players: {}", e),
    }

    // Example 8: Get all games
    println!("\nFetching all games...");
    match client.get_all_games(Some(0), Some(20)).await {
        Ok(games) => {
            println!("Found {} games", games.items.len());
            for game in games.items.iter().take(5) {
                println!("  - {} ({})", game.long_label, game.game_id);
            }
        }
        Err(e) => eprintln!("Error fetching games: {}", e),
    }

    // Example 9: Get hub details
    println!("\nFetching hub details...");
    match client.get_hub("hub-id-here", None).await {
        Ok(hub) => {
            println!("Hub: {}", hub.name);
            println!("Game: {}", hub.game_id);
        }
        Err(e) => eprintln!("Error fetching hub: {}", e),
    }

    // Example 10: Get global ranking
    println!("\nFetching global ranking...");
    match client
        .get_global_ranking("cs2", "EU", None, Some(0), Some(20))
        .await
    {
        Ok(ranking) => {
            println!("Found {} players in ranking", ranking.items.len());
            for entry in ranking.items.iter().take(5) {
                println!(
                    "  {}: {} - ELO: {}",
                    entry.position, entry.nickname, entry.faceit_elo
                );
            }
        }
        Err(e) => eprintln!("Error fetching ranking: {}", e),
    }

    // Example 11: Using the builder pattern with authentication
    println!("\nCreating client with builder...");
    let _custom_client = HttpClient::builder()
        .api_key("your-api-key-or-access-token-here")
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    println!("Client created with custom configuration");

    Ok(())
}
