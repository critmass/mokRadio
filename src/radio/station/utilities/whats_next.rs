use std::collections::BTreeSet;

use rand::seq::IndexedRandom;
use rand::rng;

use crate::radio::station::content::track::Track;


pub fn next_random(play_list:&mut  Vec<Track>) -> Option<Track> {

    let next_track = play_list.choose(&mut rng());
    return Some(next_track.unwrap().clone());
}

pub fn next_shuffle(play_list:&mut  Vec<Track>) -> Option<Track> {

    let next_track = play_list.pop();
    return next_track;
}
pub fn next_chronologic(play_list:&mut  BTreeSet<Track>) -> Option<Track> {

    let next_track = play_list.pop_first();
    return next_track;
}
pub fn next_reverse(play_list:&mut  BTreeSet<Track>) -> Option<Track> {

    let next_track = play_list.pop_last();
    return next_track;
}