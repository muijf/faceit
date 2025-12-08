use crate::error::Error;
use crate::http::Client;
use crate::types::*;

/// High-level API for interacting with a specific game
///
/// This struct provides a convenient way to work with game data
/// without needing to pass the game ID to each method call.
///
/// # Examples
///
/// ```no_run
/// # use faceit::{HttpClient, http::ergonomic::Game};
/// # async fn example() -> Result<(), faceit::error::Error> {
/// let client = HttpClient::new();
/// let game = Game::new("cs2", &client);
///
/// // Get game details
/// let game_data = game.get().await?;
/// println!("Game: {}", game_data.long_label);
///
/// // Get parent game
/// let parent = game.parent().await?;
/// # Ok(())
/// # }
/// ```
pub struct Game<'a> {
    game_id: String,
    client: &'a Client,
}

impl<'a> Game<'a> {
    /// Create a new Game instance
    ///
    /// # Arguments
    /// * `game_id` - The game ID (e.g., "cs2", "csgo")
    /// * `client` - Reference to the FACEIT client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Game};
    /// let client = HttpClient::new();
    /// let game = Game::new("cs2", &client);
    /// ```
    pub fn new(game_id: impl Into<String>, client: &'a Client) -> Self {
        Self {
            game_id: game_id.into(),
            client,
        }
    }

    /// Get the game's ID
    pub fn id(&self) -> &str {
        &self.game_id
    }

    /// Get the game's details
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Game};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let game = Game::new("cs2", &client);
    /// let game_data = game.get().await?;
    /// println!("Game: {}", game_data.long_label);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<crate::types::Game, Error> {
        self.client.get_game(&self.game_id).await
    }

    /// Get the parent game details (for region-specific games)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Game};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let game = Game::new("game-id", &client);
    /// let parent = game.parent().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn parent(&self) -> Result<crate::types::Game, Error> {
        self.client.get_parent_game(&self.game_id).await
    }

    /// Get game matchmakings
    ///
    /// # Arguments
    /// * `region` - Optional region filter
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Game};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let game = Game::new("cs2", &client);
    /// let matchmakings = game.matchmakings(Some("EU"), Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn matchmakings(
        &self,
        region: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<MatchmakingList, Error> {
        self.client
            .get_game_matchmakings(&self.game_id, region, offset, limit)
            .await
    }
}
