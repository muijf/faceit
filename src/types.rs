use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Pagination Types
// ============================================================================

/// Pagination metadata for list responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub start: i64,
    pub end: i64,
}

// ============================================================================
// Player Types
// ============================================================================

/// Player information from FACEIT API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "player_id")]
    pub player_id: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    #[serde(rename = "steam_id_64")]
    pub steam_id_64: Option<String>,
    #[serde(rename = "steam_nickname")]
    pub steam_nickname: Option<String>,
    #[serde(rename = "new_steam_id")]
    pub new_steam_id: Option<String>,
    #[serde(rename = "memberships")]
    pub memberships: Option<Vec<String>>,
    #[serde(rename = "games")]
    pub games: Option<std::collections::HashMap<String, GameDetail>>,
    #[serde(rename = "verified")]
    pub verified: Option<bool>,
    #[serde(rename = "activated_at")]
    pub activated_at: Option<DateTime<Utc>>,
    #[serde(rename = "cover_image")]
    pub cover_image: Option<String>,
    #[serde(rename = "friends_ids")]
    pub friends_ids: Option<Vec<String>>,
    #[serde(rename = "platforms")]
    pub platforms: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "settings")]
    pub settings: Option<UserSettings>,
}

/// Game-specific player details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDetail {
    #[serde(rename = "faceit_elo")]
    pub faceit_elo: Option<i64>,
    #[serde(rename = "game_player_id")]
    pub game_player_id: Option<String>,
    #[serde(rename = "game_player_name")]
    pub game_player_name: Option<String>,
    #[serde(rename = "game_profile_id")]
    pub game_profile_id: Option<String>,
    pub region: Option<String>,
    pub regions: Option<Vec<String>>,
    #[serde(rename = "skill_level")]
    pub skill_level: Option<i64>,
    #[serde(rename = "skill_level_label")]
    pub skill_level_label: Option<String>,
}

/// User settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub language: Option<String>,
}

/// Player stats for a specific game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    #[serde(rename = "player_id")]
    pub player_id: String,
    #[serde(rename = "game_id")]
    pub game_id: String,
    pub lifetime: Option<serde_json::Value>,
    pub segments: Option<Vec<serde_json::Value>>,
}

/// Player ban information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBan {
    #[serde(rename = "user_id")]
    pub user_id: String,
    pub nickname: String,
    pub game: String,
    #[serde(rename = "starts_at")]
    pub starts_at: DateTime<Utc>,
    #[serde(rename = "ends_at")]
    pub ends_at: DateTime<Utc>,
    #[serde(rename = "type")]
    pub ban_type: String,
    pub reason: String,
}

/// Player bans list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBansList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<PlayerBan>,
}

// ============================================================================
// Match Types
// ============================================================================

/// Match information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    #[serde(rename = "match_id")]
    pub match_id: String,
    pub game: String,
    pub region: Option<String>,
    #[serde(rename = "competition_id")]
    pub competition_id: Option<String>,
    #[serde(rename = "competition_type")]
    pub competition_type: Option<String>,
    #[serde(rename = "competition_name")]
    pub competition_name: Option<String>,
    #[serde(rename = "organizer_id")]
    pub organizer_id: Option<String>,
    pub teams: Option<std::collections::HashMap<String, Faction>>,
    pub status: String,
    #[serde(rename = "started_at")]
    pub started_at: Option<i64>,
    #[serde(rename = "finished_at")]
    pub finished_at: Option<i64>,
    #[serde(rename = "scheduled_at")]
    pub scheduled_at: Option<i64>,
    #[serde(rename = "configured_at")]
    pub configured_at: Option<i64>,
    #[serde(rename = "best_of")]
    pub best_of: Option<i64>,
    pub results: Option<MatchResult>,
    #[serde(rename = "detailed_results")]
    pub detailed_results: Option<Vec<DetailedMatchResult>>,
    pub round: Option<i64>,
    pub group: Option<i64>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    #[serde(rename = "chat_room_id")]
    pub chat_room_id: Option<String>,
    #[serde(rename = "demo_url")]
    pub demo_url: Option<Vec<String>>,
    #[serde(rename = "calculate_elo")]
    pub calculate_elo: Option<bool>,
    #[serde(rename = "broadcast_start_time")]
    pub broadcast_start_time: Option<i64>,
    #[serde(rename = "broadcast_start_time_label")]
    pub broadcast_start_time_label: Option<String>,
    pub version: Option<i64>,
    pub voting: Option<serde_json::Value>,
}

/// Match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub score: Option<std::collections::HashMap<String, i64>>,
    pub winner: Option<String>,
}

/// Detailed match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedMatchResult {
    #[serde(rename = "asc_score")]
    pub asc_score: Option<bool>,
    pub factions: Option<std::collections::HashMap<String, FactionResult>>,
    pub winner: Option<String>,
}

/// Faction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionResult {
    pub score: i64,
}

/// Faction (team) in a match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    #[serde(rename = "faction_id")]
    pub faction_id: Option<String>,
    pub leader: Option<String>,
    pub avatar: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub faction_type: Option<String>,
    pub roster: Option<Vec<Roster>>,
    pub stats: Option<Stats>,
    pub substituted: Option<bool>,
}

/// Roster member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roster {
    #[serde(rename = "player_id")]
    pub player_id: String,
    pub nickname: String,
    pub avatar: Option<String>,
    #[serde(rename = "game_player_id")]
    pub game_player_id: Option<String>,
    #[serde(rename = "game_player_name")]
    pub game_player_name: Option<String>,
    #[serde(rename = "game_skill_level")]
    pub game_skill_level: Option<i64>,
    #[serde(rename = "anticheat_required")]
    pub anticheat_required: Option<bool>,
    pub membership: Option<String>,
}

/// Match stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub rating: Option<i64>,
    #[serde(rename = "skillLevel")]
    pub skill_level: Option<SkillLevel>,
    #[serde(rename = "winProbability")]
    pub win_probability: Option<f64>,
}

/// Skill level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillLevel {
    pub average: Option<i64>,
    pub range: Option<SkillLevelRange>,
}

/// Skill level range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillLevelRange {
    pub min: Option<i64>,
    pub max: Option<i64>,
}

/// Match stats response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchStats {
    pub rounds: Vec<RoundStats>,
}

/// Round stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundStats {
    #[serde(rename = "match_id")]
    pub match_id: Option<String>,
    #[serde(rename = "game_id")]
    pub game_id: Option<String>,
    #[serde(rename = "competition_id")]
    pub competition_id: Option<String>,
    #[serde(rename = "game_mode")]
    pub game_mode: Option<String>,
    #[serde(rename = "match_round")]
    pub match_round: Option<i64>,
    pub played: Option<i64>,
    #[serde(rename = "best_of")]
    pub best_of: Option<i64>,
    #[serde(rename = "round_stats")]
    pub round_stats: Option<serde_json::Value>,
    pub teams: Option<Vec<TeamStatsSimple>>,
}

/// Team stats simple
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStatsSimple {
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    pub premade: Option<bool>,
    #[serde(rename = "team_stats")]
    pub team_stats: Option<serde_json::Value>,
    pub players: Option<Vec<PlayerStatsSimple>>,
}

/// Player stats simple
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStatsSimple {
    #[serde(rename = "player_id")]
    pub player_id: Option<String>,
    pub nickname: Option<String>,
    #[serde(rename = "player_stats")]
    pub player_stats: Option<serde_json::Value>,
}

/// Match history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchHistory {
    #[serde(rename = "match_id")]
    pub match_id: String,
    #[serde(rename = "game_id")]
    pub game_id: String,
    pub region: Option<String>,
    #[serde(rename = "match_type")]
    pub match_type: Option<String>,
    #[serde(rename = "game_mode")]
    pub game_mode: Option<String>,
    #[serde(rename = "max_players")]
    pub max_players: Option<i64>,
    #[serde(rename = "teams_size")]
    pub teams_size: Option<i64>,
    #[serde(rename = "teams")]
    pub teams: Option<std::collections::HashMap<String, HistoryFaction>>,
    #[serde(rename = "playing_players")]
    pub playing_players: Option<Vec<String>>,
    #[serde(rename = "competition_id")]
    pub competition_id: Option<String>,
    #[serde(rename = "competition_name")]
    pub competition_name: Option<String>,
    #[serde(rename = "competition_type")]
    pub competition_type: Option<String>,
    #[serde(rename = "organizer_id")]
    pub organizer_id: Option<String>,
    #[serde(rename = "started_at")]
    pub started_at: Option<i64>,
    #[serde(rename = "finished_at")]
    pub finished_at: Option<i64>,
    pub status: String,
    pub results: Option<MatchResult>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
}

/// History faction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFaction {
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    #[serde(rename = "type")]
    pub faction_type: Option<String>,
    pub players: Option<Vec<MatchHistoryPlayer>>,
}

/// Match history player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchHistoryPlayer {
    #[serde(rename = "player_id")]
    pub player_id: String,
    pub nickname: String,
    pub avatar: Option<String>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    #[serde(rename = "game_player_id")]
    pub game_player_id: Option<String>,
    #[serde(rename = "game_player_name")]
    pub game_player_name: Option<String>,
    #[serde(rename = "skill_level")]
    pub skill_level: Option<i64>,
}

// ============================================================================
// List Response Types
// ============================================================================

/// Games list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamesList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<Game>,
}

/// Matches list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchesList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<Match>,
}

/// Match history list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchHistoryList {
    pub start: i64,
    pub end: i64,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub items: Vec<MatchHistory>,
}

// ============================================================================
// Game Types
// ============================================================================

/// Game information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "game_id")]
    pub game_id: String,
    #[serde(rename = "short_label")]
    pub short_label: String,
    #[serde(rename = "long_label")]
    pub long_label: String,
    pub assets: Option<GameAssets>,
    pub platforms: Option<Vec<String>>,
    pub regions: Option<Vec<String>>,
    pub order: Option<i64>,
    #[serde(rename = "parent_game_id")]
    pub parent_game_id: Option<String>,
}

/// Game assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAssets {
    pub cover: Option<String>,
    #[serde(rename = "featured_img_l")]
    pub featured_img_l: Option<String>,
    #[serde(rename = "featured_img_m")]
    pub featured_img_m: Option<String>,
    #[serde(rename = "featured_img_s")]
    pub featured_img_s: Option<String>,
    #[serde(rename = "flag_img_icon")]
    pub flag_img_icon: Option<String>,
    #[serde(rename = "flag_img_l")]
    pub flag_img_l: Option<String>,
    #[serde(rename = "flag_img_m")]
    pub flag_img_m: Option<String>,
    #[serde(rename = "flag_img_s")]
    pub flag_img_s: Option<String>,
    #[serde(rename = "landing_page")]
    pub landing_page: Option<String>,
}

// ============================================================================
// Hub Types
// ============================================================================

/// Hub information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hub {
    #[serde(rename = "hub_id")]
    pub hub_id: String,
    pub name: String,
    pub avatar: Option<String>,
    #[serde(rename = "game_id")]
    pub game_id: String,
    #[serde(rename = "game_data")]
    pub game_data: Option<Game>,
    #[serde(rename = "organizer_id")]
    pub organizer_id: String,
    #[serde(rename = "organizer_data")]
    pub organizer_data: Option<Organizer>,
    pub region: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    #[serde(rename = "cover_image")]
    pub cover_image: Option<String>,
    #[serde(rename = "background_image")]
    pub background_image: Option<String>,
    #[serde(rename = "chat_room_id")]
    pub chat_room_id: Option<String>,
    #[serde(rename = "join_permission")]
    pub join_permission: Option<String>,
    #[serde(rename = "min_skill_level")]
    pub min_skill_level: Option<i64>,
    #[serde(rename = "max_skill_level")]
    pub max_skill_level: Option<i64>,
    #[serde(rename = "players_joined")]
    pub players_joined: Option<i64>,
    #[serde(rename = "rule_id")]
    pub rule_id: Option<String>,
}

/// Hub members list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubMembers {
    pub start: i64,
    pub end: i64,
    pub items: Vec<HubUser>,
}

/// Hub user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubUser {
    #[serde(rename = "user_id")]
    pub user_id: String,
    pub nickname: String,
    pub avatar: Option<String>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    pub roles: Option<Vec<String>>,
}

/// Hub stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubStats {
    #[serde(rename = "game_id")]
    pub game_id: String,
    pub players: Vec<StatsCompetitionPlayer>,
}

/// Stats competition player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsCompetitionPlayer {
    #[serde(rename = "player_id")]
    pub player_id: String,
    pub nickname: String,
    pub stats: serde_json::Value,
}

/// Hubs list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubsList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<Hub>,
}

// ============================================================================
// Championship Types
// ============================================================================

/// Championship information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Championship {
    #[serde(rename = "championship_id")]
    pub championship_id: String,
    pub id: Option<String>, // Deprecated
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "game_id")]
    pub game_id: String,
    #[serde(rename = "game_data")]
    pub game_data: Option<Game>,
    #[serde(rename = "organizer_id")]
    pub organizer_id: String,
    #[serde(rename = "organizer_data")]
    pub organizer_data: Option<Organizer>,
    pub region: Option<String>,
    pub avatar: Option<String>,
    #[serde(rename = "cover_image")]
    pub cover_image: Option<String>,
    #[serde(rename = "background_image")]
    pub background_image: Option<String>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    pub status: String,
    #[serde(rename = "championship_start")]
    pub championship_start: Option<i64>,
    #[serde(rename = "subscription_start")]
    pub subscription_start: Option<i64>,
    #[serde(rename = "subscription_end")]
    pub subscription_end: Option<i64>,
    #[serde(rename = "checkin_start")]
    pub checkin_start: Option<i64>,
    #[serde(rename = "checkin_clear")]
    pub checkin_clear: Option<i64>,
    #[serde(rename = "checkin_enabled")]
    pub checkin_enabled: Option<bool>,
    #[serde(rename = "current_subscriptions")]
    pub current_subscriptions: Option<i64>,
    pub slots: Option<i64>,
    pub full: Option<bool>,
    #[serde(rename = "subscriptions_locked")]
    pub subscriptions_locked: Option<bool>,
    pub featured: Option<bool>,
    #[serde(rename = "anticheat_required")]
    pub anticheat_required: Option<bool>,
    pub prizes: Option<Vec<Prize>>,
    #[serde(rename = "total_prizes")]
    pub total_prizes: Option<i64>,
    #[serde(rename = "total_rounds")]
    pub total_rounds: Option<i64>,
    #[serde(rename = "total_groups")]
    pub total_groups: Option<i64>,
    #[serde(rename = "seeding_strategy")]
    pub seeding_strategy: Option<String>,
    #[serde(rename = "rules_id")]
    pub rules_id: Option<String>,
    #[serde(rename = "join_checks")]
    pub join_checks: Option<JoinCheck>,
    pub schedule: Option<std::collections::HashMap<String, ChampionshipSchedule>>,
    pub screening: Option<ChampionshipScreening>,
    pub stream: Option<ChampionshipStream>,
    #[serde(rename = "substitution_configuration")]
    pub substitution_configuration: Option<SubstitutionConfiguration>,
    #[serde(rename = "type")]
    pub championship_type: Option<String>,
}

/// Prize
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prize {
    pub rank: i64,
    #[serde(rename = "faceit_points")]
    pub faceit_points: Option<i64>,
}

/// Join check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinCheck {
    #[serde(rename = "join_policy")]
    pub join_policy: Option<String>,
    #[serde(rename = "membership_type")]
    pub membership_type: Option<String>,
    #[serde(rename = "min_skill_level")]
    pub min_skill_level: Option<i64>,
    #[serde(rename = "max_skill_level")]
    pub max_skill_level: Option<i64>,
    #[serde(rename = "allowed_team_types")]
    pub allowed_team_types: Option<Vec<String>>,
    #[serde(rename = "whitelist_geo_countries")]
    pub whitelist_geo_countries: Option<Vec<String>>,
    #[serde(rename = "whitelist_geo_countries_min_players")]
    pub whitelist_geo_countries_min_players: Option<i64>,
    #[serde(rename = "blacklist_geo_countries")]
    pub blacklist_geo_countries: Option<Vec<String>>,
}

/// Championship schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionshipSchedule {
    pub date: i64,
    pub status: String,
}

/// Championship screening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionshipScreening {
    pub enabled: bool,
    pub id: String,
}

/// Championship stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionshipStream {
    pub active: bool,
    pub platform: Option<String>,
    pub source: Option<String>,
    pub title: Option<String>,
}

/// Substitution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstitutionConfiguration {
    #[serde(rename = "max_substitutes")]
    pub max_substitutes: Option<i64>,
    #[serde(rename = "max_substitutions")]
    pub max_substitutions: Option<i64>,
}

/// Championships list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionshipsList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<Championship>,
}

// ============================================================================
// Organizer Types
// ============================================================================

/// Organizer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organizer {
    #[serde(rename = "organizer_id")]
    pub organizer_id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub cover: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    pub twitter: Option<String>,
    pub youtube: Option<String>,
    pub twitch: Option<String>,
    pub facebook: Option<String>,
    pub vk: Option<String>,
    pub website: Option<String>,
    #[serde(rename = "followers_count")]
    pub followers_count: Option<i64>,
    #[serde(rename = "type")]
    pub organizer_type: Option<String>,
}

// ============================================================================
// Team Types
// ============================================================================

/// Team information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "team_id")]
    pub team_id: String,
    pub name: String,
    pub nickname: String,
    pub avatar: Option<String>,
    #[serde(rename = "cover_image")]
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub game: Option<String>,
    pub leader: Option<String>,
    pub members: Option<Vec<UserSimple>>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    #[serde(rename = "chat_room_id")]
    pub chat_room_id: Option<String>,
    pub twitter: Option<String>,
    pub youtube: Option<String>,
    pub facebook: Option<String>,
    pub website: Option<String>,
    #[serde(rename = "team_type")]
    pub team_type: Option<String>,
}

/// User simple
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSimple {
    #[serde(rename = "user_id")]
    pub user_id: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    #[serde(rename = "membership_type")]
    pub membership_type: Option<String>,
    pub memberships: Option<Vec<String>>,
    #[serde(rename = "skill_level")]
    pub skill_level: Option<i64>,
}

/// Team stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStats {
    #[serde(rename = "team_id")]
    pub team_id: String,
    #[serde(rename = "game_id")]
    pub game_id: String,
    pub lifetime: Option<serde_json::Value>,
    pub segments: Option<Vec<serde_json::Value>>,
}

/// Team list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<Team>,
}

// ============================================================================
// Search Types
// ============================================================================

/// User search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearch {
    #[serde(rename = "player_id")]
    pub player_id: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub country: Option<String>,
    pub verified: Option<bool>,
    pub status: Option<String>,
    pub games: Option<Vec<GameUserSearch>>,
}

/// Game user search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameUserSearch {
    pub name: String,
    #[serde(rename = "skill_level")]
    pub skill_level: String,
}

/// Users search list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersSearchList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<UserSearch>,
}

/// Team search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamSearch {
    #[serde(rename = "team_id")]
    pub team_id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub game: Option<String>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    #[serde(rename = "chat_room_id")]
    pub chat_room_id: Option<String>,
    pub verified: Option<bool>,
}

/// Teams search list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsSearchList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<TeamSearch>,
}

/// Competition search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionSearch {
    #[serde(rename = "competition_id")]
    pub competition_id: String,
    #[serde(rename = "competition_type")]
    pub competition_type: String,
    pub name: String,
    pub game: Option<String>,
    pub region: Option<String>,
    #[serde(rename = "organizer_id")]
    pub organizer_id: String,
    #[serde(rename = "organizer_name")]
    pub organizer_name: Option<String>,
    #[serde(rename = "organizer_type")]
    pub organizer_type: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "started_at")]
    pub started_at: Option<i64>,
    pub slots: Option<i64>,
    #[serde(rename = "number_of_members")]
    pub number_of_members: Option<i64>,
    #[serde(rename = "players_joined")]
    pub players_joined: Option<i64>,
    #[serde(rename = "players_checkedin")]
    pub players_checkedin: Option<i64>,
    #[serde(rename = "prize_type")]
    pub prize_type: Option<String>,
    #[serde(rename = "total_prize")]
    pub total_prize: Option<String>,
}

/// Competitions search list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionsSearchList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<CompetitionSearch>,
}

// ============================================================================
// Ranking Types
// ============================================================================

/// Global ranking entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalRanking {
    #[serde(rename = "player_id")]
    pub player_id: String,
    pub nickname: String,
    pub position: i64,
    #[serde(rename = "faceit_elo")]
    pub faceit_elo: i64,
    #[serde(rename = "game_skill_level")]
    pub game_skill_level: i64,
    pub country: Option<String>,
}

/// Global ranking list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalRankingList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<GlobalRanking>,
}

/// Player global ranking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGlobalRanking {
    pub position: i64,
    pub start: i64,
    pub end: i64,
    pub items: Vec<GlobalRanking>,
}

// ============================================================================
// Tournament Types
// ============================================================================

/// Tournament information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    #[serde(rename = "tournament_id")]
    pub tournament_id: String,
    #[serde(rename = "competition_id")]
    pub competition_id: Option<String>, // Deprecated
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "game_id")]
    pub game_id: String,
    #[serde(rename = "game_data")]
    pub game_data: Option<Game>,
    #[serde(rename = "organizer_id")]
    pub organizer_id: String,
    #[serde(rename = "organizer_data")]
    pub organizer_data: Option<Organizer>,
    pub region: Option<String>,
    pub status: String,
    #[serde(rename = "started_at")]
    pub started_at: Option<i64>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    #[serde(rename = "cover_image")]
    pub cover_image: Option<String>,
    #[serde(rename = "featured_image")]
    pub featured_image: Option<String>,
    #[serde(rename = "anticheat_required")]
    pub anticheat_required: Option<bool>,
    #[serde(rename = "calculate_elo")]
    pub calculate_elo: Option<bool>,
    #[serde(rename = "best_of")]
    pub best_of: Option<i64>,
    #[serde(rename = "match_type")]
    pub match_type: Option<String>,
    #[serde(rename = "invite_type")]
    pub invite_type: Option<String>,
    #[serde(rename = "membership_type")]
    pub membership_type: Option<String>,
    #[serde(rename = "min_skill")]
    pub min_skill: Option<i64>,
    #[serde(rename = "max_skill")]
    pub max_skill: Option<i64>,
    #[serde(rename = "number_of_players")]
    pub number_of_players: Option<i64>,
    #[serde(rename = "number_of_players_joined")]
    pub number_of_players_joined: Option<i64>,
    #[serde(rename = "number_of_players_checkedin")]
    pub number_of_players_checkedin: Option<i64>,
    #[serde(rename = "number_of_players_participants")]
    pub number_of_players_participants: Option<i64>,
    #[serde(rename = "team_size")]
    pub team_size: Option<i64>,
    #[serde(rename = "substitutes_allowed")]
    pub substitutes_allowed: Option<i64>,
    #[serde(rename = "substitutions_allowed")]
    pub substitutions_allowed: Option<i64>,
    #[serde(rename = "total_prize")]
    pub total_prize: Option<String>,
    #[serde(rename = "prize_type")]
    pub prize_type: Option<String>,
    pub custom: Option<bool>,
    pub rule: Option<String>,
    pub rounds: Option<Vec<serde_json::Value>>,
    pub voting: Option<serde_json::Value>,
    #[serde(rename = "whitelist_countries")]
    pub whitelist_countries: Option<Vec<String>>,
}

/// Tournaments list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentsList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<TournamentSimple>,
}

// ============================================================================
// Matchmaking Types
// ============================================================================

/// Matchmaking information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matchmaking {
    pub id: String,
    pub name: String,
    pub game: String,
    pub region: Option<String>,
    #[serde(rename = "short_description")]
    pub short_description: Option<String>,
    #[serde(rename = "long_description")]
    pub long_description: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "league_id")]
    pub league_id: Option<String>,
    pub queues: Option<Vec<MatchmakingQueue>>,
}

/// Matchmaking queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchmakingQueue {
    pub id: String,
    pub name: String,
    pub open: Option<bool>,
    pub paused: Option<bool>,
    #[serde(rename = "organizer_id")]
    pub organizer_id: Option<String>,
}

/// Matchmaking list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchmakingList {
    pub start: i64,
    pub end: i64,
    pub items: Vec<MatchmakingSlim>,
}

/// Matchmaking slim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchmakingSlim {
    pub id: String,
    pub name: String,
    pub game: String,
    pub region: Option<String>,
    #[serde(rename = "has_league")]
    pub has_league: Option<bool>,
}

/// Tournament simple
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentSimple {
    #[serde(rename = "tournament_id")]
    pub tournament_id: String,
    pub name: String,
    #[serde(rename = "game_id")]
    pub game_id: String,
    pub region: Option<String>,
    pub status: String,
    #[serde(rename = "started_at")]
    pub started_at: Option<i64>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: Option<String>,
    #[serde(rename = "featured_image")]
    pub featured_image: Option<String>,
    #[serde(rename = "anticheat_required")]
    pub anticheat_required: Option<bool>,
    pub custom: Option<bool>,
    #[serde(rename = "match_type")]
    pub match_type: Option<String>,
    #[serde(rename = "invite_type")]
    pub invite_type: Option<String>,
    #[serde(rename = "membership_type")]
    pub membership_type: Option<String>,
    #[serde(rename = "min_skill")]
    pub min_skill: Option<i64>,
    #[serde(rename = "max_skill")]
    pub max_skill: Option<i64>,
    #[serde(rename = "number_of_players")]
    pub number_of_players: Option<i64>,
    #[serde(rename = "number_of_players_joined")]
    pub number_of_players_joined: Option<i64>,
    #[serde(rename = "number_of_players_checkedin")]
    pub number_of_players_checkedin: Option<i64>,
    #[serde(rename = "number_of_players_participants")]
    pub number_of_players_participants: Option<i64>,
    #[serde(rename = "team_size")]
    pub team_size: Option<i64>,
    #[serde(rename = "total_prize")]
    pub total_prize: Option<String>,
    #[serde(rename = "prize_type")]
    pub prize_type: Option<String>,
    #[serde(rename = "organizer_id")]
    pub organizer_id: String,
    #[serde(rename = "subscriptions_count")]
    pub subscriptions_count: Option<i64>,
    #[serde(rename = "whitelist_countries")]
    pub whitelist_countries: Option<Vec<String>>,
}
