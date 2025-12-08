use crate::error::Error;
use crate::http::Client;
use crate::types::*;

/// High-level API for interacting with a specific championship
///
/// This struct provides a convenient way to work with championship data
/// without needing to pass the championship ID to each method call.
///
/// # Examples
///
/// ```no_run
/// # use faceit::{HttpClient, http::ergonomic::Championship};
/// # async fn example() -> Result<(), faceit::error::Error> {
/// let client = HttpClient::new();
/// let championship = Championship::new("championship-id-here", &client);
///
/// // Get championship details
/// let championship_data = championship.get(None).await?;
/// println!("Championship: {}", championship_data.name);
///
/// // Get championship matches
/// let matches = championship.matches(Some("all"), Some(0), Some(20)).await?;
/// # Ok(())
/// # }
/// ```
pub struct Championship<'a> {
    championship_id: String,
    client: &'a Client,
}

impl<'a> Championship<'a> {
    /// Create a new Championship instance
    ///
    /// # Arguments
    /// * `championship_id` - The championship ID
    /// * `client` - Reference to the FACEIT client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Championship};
    /// let client = HttpClient::new();
    /// let championship = Championship::new("championship-id-here", &client);
    /// ```
    pub fn new(championship_id: impl Into<String>, client: &'a Client) -> Self {
        Self {
            championship_id: championship_id.into(),
            client,
        }
    }

    /// Get the championship's ID
    pub fn id(&self) -> &str {
        &self.championship_id
    }

    /// Get the championship's details
    ///
    /// # Arguments
    /// * `expanded` - Optional list of entities to expand (e.g., ["organizer", "game"])
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Championship};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let championship = Championship::new("championship-id-here", &client);
    /// let championship_data = championship.get(None).await?;
    /// println!("Championship: {}", championship_data.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(
        &self,
        expanded: Option<&[&str]>,
    ) -> Result<crate::types::Championship, Error> {
        self.client
            .get_championship(&self.championship_id, expanded)
            .await
    }

    /// Get the championship's matches
    ///
    /// # Arguments
    /// * `match_type` - Optional match type filter ("all", "upcoming", "ongoing", "past")
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Championship};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let championship = Championship::new("championship-id-here", &client);
    /// let matches = championship.matches(Some("all"), Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn matches(
        &self,
        match_type: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<MatchesList, Error> {
        self.client
            .get_championship_matches(&self.championship_id, match_type, offset, limit)
            .await
    }
}
