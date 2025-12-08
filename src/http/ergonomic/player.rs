use crate::error::Error;
use crate::http::Client;
use crate::types::*;

/// High-level API for interacting with a specific player
///
/// This struct provides a convenient way to work with player data
/// without needing to pass the player ID to each method call.
///
/// # Examples
///
/// ```no_run
/// # use faceit::{HttpClient, http::ergonomic::Player};
/// # async fn example() -> Result<(), faceit::error::Error> {
/// let client = HttpClient::new();
/// let player = Player::new("player-id-here", &client);
///
/// // Get player details
/// let player_data = player.get().await?;
/// println!("Player: {}", player_data.nickname);
///
/// // Get stats
/// let stats = player.stats("cs2").await?;
///
/// // Get match history
/// let history = player.history("cs2", None, None, Some(0), Some(20)).await?;
/// # Ok(())
/// # }
/// ```
pub struct Player<'a> {
    player_id: String,
    client: &'a Client,
}

impl<'a> Player<'a> {
    /// Create a new Player instance
    ///
    /// # Arguments
    /// * `player_id` - The FACEIT player ID
    /// * `client` - Reference to the FACEIT client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Player};
    /// let client = HttpClient::new();
    /// let player = Player::new("player-id-here", &client);
    /// ```
    pub fn new(player_id: impl Into<String>, client: &'a Client) -> Self {
        Self {
            player_id: player_id.into(),
            client,
        }
    }

    /// Get the player's ID
    pub fn id(&self) -> &str {
        &self.player_id
    }

    /// Get the player's details
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Player};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let player = Player::new("player-id-here", &client);
    /// let player_data = player.get().await?;
    /// println!("Player: {}", player_data.nickname);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<crate::types::Player, Error> {
        self.client.get_player(&self.player_id).await
    }

    /// Get the player's statistics for a specific game
    ///
    /// # Arguments
    /// * `game_id` - The game ID (e.g., "cs2", "csgo")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Player};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let player = Player::new("player-id-here", &client);
    /// let stats = player.stats("cs2").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stats(&self, game_id: &str) -> Result<PlayerStats, Error> {
        self.client.get_player_stats(&self.player_id, game_id).await
    }

    /// Get the player's match history
    ///
    /// # Arguments
    /// * `game` - The game ID (required)
    /// * `from` - Optional start timestamp (Unix time)
    /// * `to` - Optional end timestamp (Unix time)
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Player};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let player = Player::new("player-id-here", &client);
    /// let history = player.history("cs2", None, None, Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn history(
        &self,
        game: &str,
        from: Option<i64>,
        to: Option<i64>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<MatchHistoryList, Error> {
        self.client
            .get_player_history(&self.player_id, game, from, to, offset, limit)
            .await
    }

    /// Get the player's bans
    ///
    /// # Arguments
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Player};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let player = Player::new("player-id-here", &client);
    /// let bans = player.bans(Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn bans(
        &self,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<PlayerBansList, Error> {
        self.client
            .get_player_bans(&self.player_id, offset, limit)
            .await
    }

    /// Get the player's hubs
    ///
    /// # Arguments
    /// * `offset` - Optional offset for pagination (default: 0, max: 1000)
    /// * `limit` - Optional limit for pagination (default: 50, max: 50)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Player};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let player = Player::new("player-id-here", &client);
    /// let hubs = player.hubs(Some(0), Some(50)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn hubs(&self, offset: Option<i64>, limit: Option<i64>) -> Result<HubsList, Error> {
        self.client
            .get_player_hubs(&self.player_id, offset, limit)
            .await
    }

    /// Get the player's teams
    ///
    /// # Arguments
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Player};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let player = Player::new("player-id-here", &client);
    /// let teams = player.teams(Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn teams(&self, offset: Option<i64>, limit: Option<i64>) -> Result<TeamList, Error> {
        self.client
            .get_player_teams(&self.player_id, offset, limit)
            .await
    }

    /// Get the player's tournaments
    ///
    /// # Arguments
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Player};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let player = Player::new("player-id-here", &client);
    /// let tournaments = player.tournaments(Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tournaments(
        &self,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<TournamentsList, Error> {
        self.client
            .get_player_tournaments(&self.player_id, offset, limit)
            .await
    }
}
