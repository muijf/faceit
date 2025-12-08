use crate::error::Error;
use crate::types::*;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://open.faceit.com";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Builder for creating a customized [`Client`]
pub struct ClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    timeout: Option<Duration>,
    client_builder: reqwest::ClientBuilder,
}

impl ClientBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self {
            base_url: None,
            api_key: None,
            timeout: Some(DEFAULT_TIMEOUT),
            client_builder: reqwest::Client::builder(),
        }
    }

    /// Set a custom base URL for the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use faceit::HttpClient;
    ///
    /// let client = HttpClient::builder()
    ///     .base_url("https://custom-api.example.com")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the API key or access token
    ///
    /// For the Data API, you can use either:
    /// - An API key (obtained from the FACEIT Developer Portal)
    /// - An access token (obtained via OAuth2)
    ///
    /// Both are passed in the `Authorization: Bearer {token}` header.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use faceit::HttpClient;
    ///
    /// // Using an API key
    /// let client = HttpClient::builder()
    ///     .api_key("your-api-key")
    ///     .build()
    ///     .unwrap();
    ///
    /// // Using an access token
    /// let client = HttpClient::builder()
    ///     .api_key("your-access-token")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set the request timeout
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use faceit::HttpClient;
    /// use std::time::Duration;
    ///
    /// let client = HttpClient::builder()
    ///     .timeout(Duration::from_secs(60))
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self.client_builder = self.client_builder.timeout(timeout);
        self
    }

    /// Configure the underlying reqwest client builder
    ///
    /// This allows advanced configuration of the HTTP client.
    pub fn client_builder(mut self, builder: reqwest::ClientBuilder) -> Self {
        self.client_builder = builder;
        self
    }

    /// Build the client
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the underlying HTTP client fails to build.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use faceit::HttpClient;
    ///
    /// let client = HttpClient::builder()
    ///     .api_key("your-api-key-or-access-token")
    ///     .build()?;
    /// # Ok::<(), faceit::error::Error>(())
    /// ```
    pub fn build(self) -> Result<Client, Error> {
        let client = self
            .client_builder
            .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
            .build()
            .map_err(Error::Http)?;

        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        Ok(Client {
            reqwest_client: client,
            base_url,
            api_key: self.api_key,
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Client for interacting with the FACEIT Public API
pub struct Client {
    reqwest_client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl Client {
    /// Create a new client without authentication
    ///
    /// Requests without authentication are subject to standard rate limits.
    ///
    /// # Panics
    ///
    /// Panics if the underlying HTTP client fails to build. This should only happen
    /// in exceptional circumstances (e.g., invalid TLS configuration). For more
    /// control over error handling, use [`builder`](Self::builder) instead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use faceit::HttpClient;
    ///
    /// let client = HttpClient::new();
    /// ```
    pub fn new() -> Self {
        ClientBuilder::new()
            .build()
            .expect("Failed to create default client")
    }

    /// Create a builder for customizing the client configuration
    ///
    /// Returns a [`ClientBuilder`] for configuring the client.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use faceit::HttpClient;
    /// use std::time::Duration;
    ///
    /// let client = HttpClient::builder()
    ///     .api_key("your-api-key-or-access-token")
    ///     .timeout(Duration::from_secs(60))
    ///     .base_url("https://custom-api.example.com")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    // ============================================================================
    // Player Methods
    // ============================================================================

    /// Get player details by player ID
    ///
    /// Returns a [`Player`](crate::types::Player) struct with player information.
    ///
    /// # Arguments
    /// * `player_id` - The FACEIT player ID (UUID format)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response (e.g., 404 if player not found).
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let player = client.get_player("player-id-here").await?;
    /// println!("Player: {}", player.nickname);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_player(&self, player_id: &str) -> Result<Player, Error> {
        let url = format!("{}/data/v4/players/{}", self.base_url, player_id);
        let request = self.reqwest_client.get(&url);
        let request = self.add_api_key_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get player details from lookup (by nickname, game, or game_player_id)
    ///
    /// Returns a [`Player`](crate::types::Player) struct with player information.
    ///
    /// # Arguments
    /// * `nickname` - Optional player nickname
    /// * `game` - Optional game ID
    /// * `game_player_id` - Optional game-specific player ID
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let player = client.get_player_from_lookup(Some("player_nickname"), None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_player_from_lookup(
        &self,
        nickname: Option<&str>,
        game: Option<&str>,
        game_player_id: Option<&str>,
    ) -> Result<Player, Error> {
        let url = format!("{}/data/v4/players", self.base_url);
        let mut request = self.reqwest_client.get(&url);

        if let Some(nickname) = nickname {
            request = request.query(&[("nickname", nickname)]);
        }
        if let Some(game) = game {
            request = request.query(&[("game", game)]);
        }
        if let Some(game_player_id) = game_player_id {
            request = request.query(&[("game_player_id", game_player_id)]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get player statistics for a specific game
    ///
    /// # Arguments
    /// * `player_id` - The FACEIT player ID
    /// * `game_id` - The game ID (e.g., "cs2", "csgo")
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let stats = client.get_player_stats("player-id", "cs2").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_player_stats(
        &self,
        player_id: &str,
        game_id: &str,
    ) -> Result<PlayerStats, Error> {
        let url = format!(
            "{}/data/v4/players/{}/stats/{}",
            self.base_url, player_id, game_id
        );
        let request = self.reqwest_client.get(&url);
        let request = self.add_api_key_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get player match history
    ///
    /// Returns a [`MatchHistoryList`](crate::types::MatchHistoryList) containing match history entries.
    ///
    /// # Arguments
    /// * `player_id` - The FACEIT player ID
    /// * `game` - The game ID (required)
    /// * `from` - Optional start timestamp (Unix time)
    /// * `to` - Optional end timestamp (Unix time)
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let history = client.get_player_history("player-id", "cs2", None, None, Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_player_history(
        &self,
        player_id: &str,
        game: &str,
        from: Option<i64>,
        to: Option<i64>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<MatchHistoryList, Error> {
        let url = format!("{}/data/v4/players/{}/history", self.base_url, player_id);
        let mut request = self.reqwest_client.get(&url);

        request = request.query(&[("game", game)]);
        if let Some(from) = from {
            request = request.query(&[("from", &from.to_string())]);
        }
        if let Some(to) = to {
            request = request.query(&[("to", &to.to_string())]);
        }
        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get player bans
    ///
    /// Returns a [`PlayerBansList`](crate::types::PlayerBansList) containing ban information.
    ///
    /// # Arguments
    /// * `player_id` - The FACEIT player ID
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let bans = client.get_player_bans("player-id", Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_player_bans(
        &self,
        player_id: &str,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<PlayerBansList, Error> {
        let url = format!("{}/data/v4/players/{}/bans", self.base_url, player_id);
        let mut request = self.reqwest_client.get(&url);

        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get player hubs
    ///
    /// Returns a [`HubsList`](crate::types::HubsList) containing hub information.
    ///
    /// # Arguments
    /// * `player_id` - The FACEIT player ID
    /// * `offset` - Optional offset for pagination (default: 0, max: 1000)
    /// * `limit` - Optional limit for pagination (default: 50, max: 50)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let hubs = client.get_player_hubs("player-id", Some(0), Some(50)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_player_hubs(
        &self,
        player_id: &str,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<HubsList, Error> {
        let url = format!("{}/data/v4/players/{}/hubs", self.base_url, player_id);
        let mut request = self.reqwest_client.get(&url);

        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get player teams
    ///
    /// Returns a [`TeamList`](crate::types::TeamList) containing team information.
    ///
    /// # Arguments
    /// * `player_id` - The FACEIT player ID
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let teams = client.get_player_teams("player-id", Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_player_teams(
        &self,
        player_id: &str,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<TeamList, Error> {
        let url = format!("{}/data/v4/players/{}/teams", self.base_url, player_id);
        let mut request = self.reqwest_client.get(&url);

        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get player tournaments
    ///
    /// Returns a [`TournamentsList`](crate::types::TournamentsList) containing tournament information.
    ///
    /// # Arguments
    /// * `player_id` - The FACEIT player ID
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let tournaments = client.get_player_tournaments("player-id", Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_player_tournaments(
        &self,
        player_id: &str,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<TournamentsList, Error> {
        let url = format!(
            "{}/data/v4/players/{}/tournaments",
            self.base_url, player_id
        );
        let mut request = self.reqwest_client.get(&url);

        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    // ============================================================================
    // Match Methods
    // ============================================================================

    /// Get match details
    ///
    /// Returns a [`Match`](crate::types::Match) struct with match information.
    ///
    /// # Arguments
    /// * `match_id` - The FACEIT match ID
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let match_details = client.get_match("match-id-here").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_match(&self, match_id: &str) -> Result<Match, Error> {
        let url = format!("{}/data/v4/matches/{}", self.base_url, match_id);
        let request = self.reqwest_client.get(&url);
        let request = self.add_api_key_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get match statistics
    ///
    /// Returns a [`MatchStats`](crate::types::MatchStats) struct with detailed match statistics.
    ///
    /// # Arguments
    /// * `match_id` - The FACEIT match ID
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let stats = client.get_match_stats("match-id-here").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_match_stats(&self, match_id: &str) -> Result<MatchStats, Error> {
        let url = format!("{}/data/v4/matches/{}/stats", self.base_url, match_id);
        let request = self.reqwest_client.get(&url);
        let request = self.add_api_key_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    // ============================================================================
    // Game Methods
    // ============================================================================

    /// Get all games
    ///
    /// # Arguments
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let games = client.get_all_games(Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_all_games(
        &self,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<GamesList, Error> {
        let url = format!("{}/data/v4/games", self.base_url);
        let mut request = self.reqwest_client.get(&url);

        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get game details
    ///
    /// Returns a [`Game`](crate::types::Game) struct with game information.
    ///
    /// # Arguments
    /// * `game_id` - The game ID (e.g., "cs2", "csgo")
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let game = client.get_game("cs2").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_game(&self, game_id: &str) -> Result<Game, Error> {
        let url = format!("{}/data/v4/games/{}", self.base_url, game_id);
        let request = self.reqwest_client.get(&url);
        let request = self.add_api_key_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get parent game details (for region-specific games)
    ///
    /// Returns a [`Game`](crate::types::Game) struct with parent game information.
    ///
    /// # Arguments
    /// * `game_id` - The game ID
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let parent_game = client.get_parent_game("game-id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_parent_game(&self, game_id: &str) -> Result<Game, Error> {
        let url = format!("{}/data/v4/games/{}/parent", self.base_url, game_id);
        let request = self.reqwest_client.get(&url);
        let request = self.add_api_key_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get game matchmakings
    ///
    /// Returns a [`MatchmakingList`](crate::types::MatchmakingList) containing matchmaking information.
    ///
    /// # Arguments
    /// * `game_id` - The game ID
    /// * `region` - Optional region filter
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let matchmakings = client.get_game_matchmakings("cs2", Some("EU"), Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_game_matchmakings(
        &self,
        game_id: &str,
        region: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<MatchmakingList, Error> {
        let url = format!("{}/data/v4/games/{}/matchmakings", self.base_url, game_id);
        let mut request = self.reqwest_client.get(&url);

        if let Some(region) = region {
            request = request.query(&[("region", region)]);
        }
        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    // ============================================================================
    // Hub Methods
    // ============================================================================

    /// Get hub details
    ///
    /// # Arguments
    /// * `hub_id` - The hub ID
    /// * `expanded` - Optional list of entities to expand (e.g., ["organizer", "game"])
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let hub = client.get_hub("hub-id", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_hub(&self, hub_id: &str, expanded: Option<&[&str]>) -> Result<Hub, Error> {
        let url = format!("{}/data/v4/hubs/{}", self.base_url, hub_id);
        let mut request = self.reqwest_client.get(&url);

        if let Some(expanded) = expanded {
            let expanded_str = expanded.join(",");
            request = request.query(&[("expanded", expanded_str.as_str())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get hub matches
    ///
    /// Returns a [`MatchesList`](crate::types::MatchesList) containing match information.
    ///
    /// # Arguments
    /// * `hub_id` - The hub ID
    /// * `match_type` - Optional match type filter ("all", "upcoming", "ongoing", "past")
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let matches = client.get_hub_matches("hub-id", Some("all"), Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_hub_matches(
        &self,
        hub_id: &str,
        match_type: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<MatchesList, Error> {
        let url = format!("{}/data/v4/hubs/{}/matches", self.base_url, hub_id);
        let mut request = self.reqwest_client.get(&url);

        if let Some(match_type) = match_type {
            request = request.query(&[("type", match_type)]);
        }
        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get hub members
    ///
    /// Returns a [`HubMembers`](crate::types::HubMembers) containing member information.
    ///
    /// # Arguments
    /// * `hub_id` - The hub ID
    /// * `offset` - Optional offset for pagination (default: 0, max: 1000)
    /// * `limit` - Optional limit for pagination (default: 50, max: 50)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let members = client.get_hub_members("hub-id", Some(0), Some(50)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_hub_members(
        &self,
        hub_id: &str,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<HubMembers, Error> {
        let url = format!("{}/data/v4/hubs/{}/members", self.base_url, hub_id);
        let mut request = self.reqwest_client.get(&url);

        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get hub statistics
    ///
    /// Returns a [`HubStats`](crate::types::HubStats) containing hub statistics.
    ///
    /// # Arguments
    /// * `hub_id` - The hub ID
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let stats = client.get_hub_stats("hub-id", Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_hub_stats(
        &self,
        hub_id: &str,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<HubStats, Error> {
        let url = format!("{}/data/v4/hubs/{}/stats", self.base_url, hub_id);
        let mut request = self.reqwest_client.get(&url);

        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    // ============================================================================
    // Championship Methods
    // ============================================================================

    /// Get championships for a game
    ///
    /// Returns a [`ChampionshipsList`](crate::types::ChampionshipsList) containing championship information.
    ///
    /// # Arguments
    /// * `game` - The game ID (required)
    /// * `championship_type` - Optional type filter ("all", "upcoming", "ongoing", "past")
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 10, max: 10)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let championships = client.get_championships("cs2", Some("all"), Some(0), Some(10)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_championships(
        &self,
        game: &str,
        championship_type: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<ChampionshipsList, Error> {
        let url = format!("{}/data/v4/championships", self.base_url);
        let mut request = self.reqwest_client.get(&url);

        request = request.query(&[("game", game)]);
        if let Some(championship_type) = championship_type {
            request = request.query(&[("type", championship_type)]);
        }
        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get championship details
    ///
    /// Returns a [`Championship`](crate::types::Championship) struct with championship information.
    ///
    /// # Arguments
    /// * `championship_id` - The championship ID
    /// * `expanded` - Optional list of entities to expand (e.g., ["organizer", "game"])
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let championship = client.get_championship("championship-id", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_championship(
        &self,
        championship_id: &str,
        expanded: Option<&[&str]>,
    ) -> Result<Championship, Error> {
        let url = format!(
            "{}/data/v4/championships/{}",
            self.base_url, championship_id
        );
        let mut request = self.reqwest_client.get(&url);

        if let Some(expanded) = expanded {
            let expanded_str = expanded.join(",");
            request = request.query(&[("expanded", expanded_str.as_str())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get championship matches
    ///
    /// Returns a [`MatchesList`](crate::types::MatchesList) containing match information.
    ///
    /// # Arguments
    /// * `championship_id` - The championship ID
    /// * `match_type` - Optional match type filter ("all", "upcoming", "ongoing", "past")
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let matches = client.get_championship_matches("championship-id", Some("all"), Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_championship_matches(
        &self,
        championship_id: &str,
        match_type: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<MatchesList, Error> {
        let url = format!(
            "{}/data/v4/championships/{}/matches",
            self.base_url, championship_id
        );
        let mut request = self.reqwest_client.get(&url);

        if let Some(match_type) = match_type {
            request = request.query(&[("type", match_type)]);
        }
        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    // ============================================================================
    // Search Methods
    // ============================================================================

    /// Search for players
    ///
    /// Returns a [`UsersSearchList`](crate::types::UsersSearchList) containing search results.
    ///
    /// # Arguments
    /// * `nickname` - Player nickname to search for (required)
    /// * `game` - Optional game ID filter
    /// * `country` - Optional country code filter (ISO 3166-1)
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let results = client.search_players("player_name", Some("cs2"), None, Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_players(
        &self,
        nickname: &str,
        game: Option<&str>,
        country: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<UsersSearchList, Error> {
        let url = format!("{}/data/v4/search/players", self.base_url);
        let mut request = self.reqwest_client.get(&url);

        request = request.query(&[("nickname", nickname)]);
        if let Some(game) = game {
            request = request.query(&[("game", game)]);
        }
        if let Some(country) = country {
            request = request.query(&[("country", country)]);
        }
        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Search for teams
    ///
    /// Returns a [`TeamsSearchList`](crate::types::TeamsSearchList) containing search results.
    ///
    /// # Arguments
    /// * `nickname` - Team nickname to search for (required)
    /// * `game` - Optional game ID filter
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let results = client.search_teams("team_name", Some("cs2"), Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_teams(
        &self,
        nickname: &str,
        game: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<TeamsSearchList, Error> {
        let url = format!("{}/data/v4/search/teams", self.base_url);
        let mut request = self.reqwest_client.get(&url);

        request = request.query(&[("nickname", nickname)]);
        if let Some(game) = game {
            request = request.query(&[("game", game)]);
        }
        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Search for hubs
    ///
    /// Returns a [`CompetitionsSearchList`](crate::types::CompetitionsSearchList) containing search results.
    ///
    /// # Arguments
    /// * `name` - Hub name to search for (required)
    /// * `game` - Optional game ID filter
    /// * `region` - Optional region filter
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let results = client.search_hubs("hub_name", Some("cs2"), Some("EU"), Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_hubs(
        &self,
        name: &str,
        game: Option<&str>,
        region: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<CompetitionsSearchList, Error> {
        let url = format!("{}/data/v4/search/hubs", self.base_url);
        let mut request = self.reqwest_client.get(&url);

        request = request.query(&[("name", name)]);
        if let Some(game) = game {
            request = request.query(&[("game", game)]);
        }
        if let Some(region) = region {
            request = request.query(&[("region", region)]);
        }
        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    // ============================================================================
    // Ranking Methods
    // ============================================================================

    /// Get global ranking for a game and region
    ///
    /// Returns a [`GlobalRankingList`](crate::types::GlobalRankingList) containing ranking information.
    ///
    /// # Arguments
    /// * `game_id` - The game ID
    /// * `region` - The region (required)
    /// * `country` - Optional country code filter (ISO 3166-1)
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let ranking = client.get_global_ranking("cs2", "EU", None, Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_global_ranking(
        &self,
        game_id: &str,
        region: &str,
        country: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<GlobalRankingList, Error> {
        let url = format!(
            "{}/data/v4/rankings/games/{}/regions/{}",
            self.base_url, game_id, region
        );
        let mut request = self.reqwest_client.get(&url);

        if let Some(country) = country {
            request = request.query(&[("country", country)]);
        }
        if let Some(offset) = offset {
            request = request.query(&[("offset", &offset.to_string())]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get player ranking in global ranking
    ///
    /// Returns a [`PlayerGlobalRanking`](crate::types::PlayerGlobalRanking) containing player ranking information.
    ///
    /// # Arguments
    /// * `game_id` - The game ID
    /// * `region` - The region (required)
    /// * `player_id` - The player ID (required)
    /// * `country` - Optional country code filter (ISO 3166-1)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the HTTP request fails.
    /// Returns [`Error::Api`] if the API returns an error response.
    /// Returns [`Error::Json`] if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::HttpClient;
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let ranking = client.get_player_ranking("cs2", "EU", "player-id", None, Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_player_ranking(
        &self,
        game_id: &str,
        region: &str,
        player_id: &str,
        country: Option<&str>,
        limit: Option<i64>,
    ) -> Result<PlayerGlobalRanking, Error> {
        let url = format!(
            "{}/data/v4/rankings/games/{}/regions/{}/players/{}",
            self.base_url, game_id, region, player_id
        );
        let mut request = self.reqwest_client.get(&url);

        if let Some(country) = country {
            request = request.query(&[("country", country)]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }

        let request = self.add_api_key_header(request);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    // ============================================================================
    // Helper Methods
    // ============================================================================

    fn add_api_key_header(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(ref api_key) = self.api_key {
            request.header("Authorization", format!("Bearer {}", api_key.as_str()))
        } else {
            request
        }
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            let status_code = status.as_u16();
            return match status_code {
                400 => Err(Error::Api(
                    status_code,
                    format!("Bad request: {}", response_text),
                )),
                401 => Err(Error::InvalidApiKey),
                403 => Err(Error::Api(
                    status_code,
                    format!("Forbidden: {}", response_text),
                )),
                404 => Err(Error::Api(
                    status_code,
                    format!("Not found: {}", response_text),
                )),
                429 => Err(Error::Api(
                    status_code,
                    format!("Too many requests: {}", response_text),
                )),
                500 => Err(Error::ServerError),
                503 => Err(Error::Api(
                    status_code,
                    format!("Service temporarily unavailable: {}", response_text),
                )),
                _ => Err(Error::Api(status_code, response_text)),
            };
        }

        // Try to parse JSON, but provide better error message if it fails
        match serde_json::from_str::<T>(&response_text) {
            Ok(json) => Ok(json),
            Err(e) => {
                // If JSON parsing fails, create a more descriptive error
                // We'll wrap it in an Api error with the response text
                Err(Error::Api(
                    status.as_u16(),
                    format!(
                        "Failed to parse JSON response: {}. Response body: {}",
                        e, response_text
                    ),
                ))
            }
        }
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the API key if set
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let builder = ClientBuilder::new();
        // Test that builder can be created and configured
        let client = builder
            .base_url("https://test.example.com")
            .api_key("test-key")
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();
        assert_eq!(client.base_url(), "https://test.example.com");
        assert_eq!(client.api_key(), Some("test-key"));
    }

    #[test]
    fn test_client_builder_with_options() {
        let client = ClientBuilder::new()
            .base_url("https://test.example.com")
            .api_key("test-key")
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        assert_eq!(client.base_url(), "https://test.example.com");
        assert_eq!(client.api_key(), Some("test-key"));
    }

    #[test]
    fn test_client_default_base_url() {
        let client = Client::new();
        assert_eq!(client.base_url(), "https://open.faceit.com");
    }

    #[test]
    fn test_player_id_string() {
        // FACEIT uses simple string player IDs (UUID format)
        let player_id = "5ea07280-2399-4c7e-88ab-f2f7db0c449f";
        assert!(!player_id.is_empty());
        // Just verify it's a valid string
        assert_eq!(player_id.len(), 36);
    }
}
