use crate::error::Error;
use crate::http::Client;

/// High-level API for interacting with a specific match
///
/// This struct provides a convenient way to work with match data
/// without needing to pass the match ID to each method call.
///
/// # Examples
///
/// ```no_run
/// # use faceit::{HttpClient, http::ergonomic::Match};
/// # async fn example() -> Result<(), faceit::error::Error> {
/// let client = HttpClient::new();
/// let match_obj = Match::new("match-id-here", &client);
///
/// // Get match details
/// let match_data = match_obj.get().await?;
/// println!("Match: {}", match_data.match_id);
///
/// // Get match stats
/// let stats = match_obj.stats().await?;
/// # Ok(())
/// # }
/// ```
pub struct Match<'a> {
    match_id: String,
    client: &'a Client,
}

impl<'a> Match<'a> {
    /// Create a new Match instance
    ///
    /// # Arguments
    /// * `match_id` - The FACEIT match ID
    /// * `client` - Reference to the FACEIT client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Match};
    /// let client = HttpClient::new();
    /// let match_obj = Match::new("match-id-here", &client);
    /// ```
    pub fn new(match_id: impl Into<String>, client: &'a Client) -> Self {
        Self {
            match_id: match_id.into(),
            client,
        }
    }

    /// Get the match's ID
    pub fn id(&self) -> &str {
        &self.match_id
    }

    /// Get the match's details
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Match};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let match_obj = Match::new("match-id-here", &client);
    /// let match_data = match_obj.get().await?;
    /// println!("Match: {}", match_data.match_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<crate::types::Match, Error> {
        self.client.get_match(&self.match_id).await
    }

    /// Get the match's statistics
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Match};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let match_obj = Match::new("match-id-here", &client);
    /// let stats = match_obj.stats().await?;
    /// println!("Rounds: {}", stats.rounds.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stats(&self) -> Result<crate::types::MatchStats, Error> {
        self.client.get_match_stats(&self.match_id).await
    }
}
