//! Playlist Selection Utilities
//! 
//! Helper functions for selecting the next track from different playlist types.
//! Each function implements the selection logic for one playlist strategy:
//! - Random: Pick any track, keep in list
//! - Shuffle: Pop tracks from shuffled list
//! - Chronologic: Pop oldest track (by file modification time)
//! - Reverse: Pop newest track (by file modification time)

use std::collections::BTreeSet;
use rand::seq::IndexedRandom;
use rand::rng;

use crate::radio::station::content::track::Track;

/// Selects a random track from the playlist without removing it
/// 
/// Used by PlayType::Random - tracks can be played multiple times.
/// The same track may be selected again on the next call.
/// 
/// # Arguments
/// * `play_list` - Mutable reference to track vector (not modified)
/// 
/// # Returns
/// - `Some(Track)` - Randomly selected track (cloned from list)
/// - `None` - Playlist is empty
/// 
/// # Behavior
/// - Track remains in the playlist after selection
/// - Each call is independent - no memory of what was played last
/// - All tracks have equal probability of selection
/// 
/// # Note
/// Currently unwraps the Option from choose(), which will panic on empty list.
/// TODO: Handle empty playlist gracefully
pub fn next_random(play_list: &mut Vec<Track>) -> Option<Track> {
    // Choose returns Option<&Track>, so we clone it to return owned Track
    let next_track = play_list.choose(&mut rng());
    Some(next_track.unwrap().clone())
}

/// Removes and returns the last track from a shuffled playlist
/// 
/// Used by PlayType::Shuffle - tracks are removed as played.
/// The playlist was pre-shuffled when created, so popping from the
/// end gives us the next random track in the predetermined order.
/// 
/// # Arguments
/// * `play_list` - Mutable reference to shuffled track vector
/// 
/// # Returns
/// - `Some(Track)` - Next track from the shuffled list
/// - `None` - Playlist is empty (all tracks played)
/// 
/// # Behavior
/// - Removes track from playlist (won't be replayed)
/// - When playlist is empty, Station reloads and reshuffles it
/// - Popping from the end is O(1) - more efficient than removing from front
pub fn next_shuffle(play_list: &mut Vec<Track>) -> Option<Track> {
    play_list.pop()
}

/// Removes and returns the oldest track (earliest file modification time)
/// 
/// Used by PlayType::Chronologic - plays tracks in order from oldest to newest.
/// BTreeSet maintains tracks sorted by modification time.
/// 
/// # Arguments
/// * `play_list` - Mutable reference to BTreeSet of tracks (sorted by time)
/// 
/// # Returns
/// - `Some(Track)` - Oldest unplayed track
/// - `None` - All tracks have been played
/// 
/// # Behavior
/// - Removes track from playlist (won't be replayed)
/// - When playlist is empty, Station goes off-air (no reload)
/// - Useful for playing content in "release order" or processing
///   files in the order they were created
pub fn next_chronologic(play_list: &mut BTreeSet<Track>) -> Option<Track> {
    play_list.pop_first()
}

/// Removes and returns the newest track (latest file modification time)
/// 
/// Used by PlayType::Reverse - plays tracks from newest to oldest.
/// BTreeSet maintains tracks sorted by modification time.
/// 
/// # Arguments
/// * `play_list` - Mutable reference to BTreeSet of tracks (sorted by time)
/// 
/// # Returns
/// - `Some(Track)` - Newest unplayed track
/// - `None` - All tracks have been played
/// 
/// # Behavior
/// - Removes track from playlist (won't be replayed)
/// - When playlist is empty, Station goes off-air (no reload)
/// - Useful for playing "latest episodes first" or prioritizing
///   recently added content
pub fn next_reverse(play_list: &mut BTreeSet<Track>) -> Option<Track> {
    play_list.pop_last()
}
