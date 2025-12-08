use crate::error::Error;
use crate::http::Client;
use crate::types::*;

/// High-level API for interacting with a specific hub
///
/// This struct provides a convenient way to work with hub data
/// without needing to pass the hub ID to each method call.
///
/// # Examples
///
/// ```no_run
/// # use faceit::{HttpClient, http::ergonomic::Hub};
/// # async fn example() -> Result<(), faceit::error::Error> {
/// let client = HttpClient::new();
/// let hub = Hub::new("hub-id-here", &client);
///
/// // Get hub details
/// let hub_data = hub.get(None).await?;
/// println!("Hub: {}", hub_data.name);
///
/// // Get hub matches
/// let matches = hub.matches(Some("all"), Some(0), Some(20)).await?;
/// # Ok(())
/// # }
/// ```
pub struct Hub<'a> {
    hub_id: String,
    client: &'a Client,
}

impl<'a> Hub<'a> {
    /// Create a new Hub instance
    ///
    /// # Arguments
    /// * `hub_id` - The hub ID
    /// * `client` - Reference to the FACEIT client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Hub};
    /// let client = HttpClient::new();
    /// let hub = Hub::new("hub-id-here", &client);
    /// ```
    pub fn new(hub_id: impl Into<String>, client: &'a Client) -> Self {
        Self {
            hub_id: hub_id.into(),
            client,
        }
    }

    /// Get the hub's ID
    pub fn id(&self) -> &str {
        &self.hub_id
    }

    /// Get the hub's details
    ///
    /// # Arguments
    /// * `expanded` - Optional list of entities to expand (e.g., ["organizer", "game"])
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Hub};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let hub = Hub::new("hub-id-here", &client);
    /// let hub_data = hub.get(None).await?;
    /// println!("Hub: {}", hub_data.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, expanded: Option<&[&str]>) -> Result<crate::types::Hub, Error> {
        self.client.get_hub(&self.hub_id, expanded).await
    }

    /// Get the hub's matches
    ///
    /// # Arguments
    /// * `match_type` - Optional match type filter ("all", "upcoming", "ongoing", "past")
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Hub};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let hub = Hub::new("hub-id-here", &client);
    /// let matches = hub.matches(Some("all"), Some(0), Some(20)).await?;
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
            .get_hub_matches(&self.hub_id, match_type, offset, limit)
            .await
    }

    /// Get the hub's members
    ///
    /// # Arguments
    /// * `offset` - Optional offset for pagination (default: 0, max: 1000)
    /// * `limit` - Optional limit for pagination (default: 50, max: 50)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Hub};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let hub = Hub::new("hub-id-here", &client);
    /// let members = hub.members(Some(0), Some(50)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn members(
        &self,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<HubMembers, Error> {
        self.client
            .get_hub_members(&self.hub_id, offset, limit)
            .await
    }

    /// Get the hub's statistics
    ///
    /// # Arguments
    /// * `offset` - Optional offset for pagination (default: 0)
    /// * `limit` - Optional limit for pagination (default: 20, max: 100)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use faceit::{HttpClient, http::ergonomic::Hub};
    /// # async fn example() -> Result<(), faceit::error::Error> {
    /// let client = HttpClient::new();
    /// let hub = Hub::new("hub-id-here", &client);
    /// let stats = hub.stats(Some(0), Some(20)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stats(&self, offset: Option<i64>, limit: Option<i64>) -> Result<HubStats, Error> {
        self.client.get_hub_stats(&self.hub_id, offset, limit).await
    }
}
