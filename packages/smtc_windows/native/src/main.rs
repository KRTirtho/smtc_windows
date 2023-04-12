use std::{thread, time::Duration};

mod api;
mod config;
mod metadata;
mod timeline;

use crate::{
    api::{initialize_media_player, update_metadata},
    config::SMTCConfig,
    metadata::MusicMetadata,
    timeline::PlaybackTimeline,
};

fn main() {
    let smtc_config = SMTCConfig::default();
    let timeline = PlaybackTimeline {
        start_time_ms: 0,
        end_time_ms: 1000,
        position_ms: 0,
        max_seek_time_ms: None,
        min_seek_time_ms: None,
    };
    let music_metadata = MusicMetadata{
      album: "Bangerz".to_string(),
      album_artist: "Miley Cyrus".to_string(),
      artist: "Miley Cyrus".to_string(),
      title: "Wrecking Ball".to_string(),
      track_number: 1,
      thumbnail: Some("https://media.glamour.com/photos/5f4c44e20c71c58fc210d35f/master/w_2560%2Cc_limit/mgid_ao_image_mtv.jpg".to_string()),
    };

    println!("Waiting 10 seconds...");

    initialize_media_player(smtc_config, timeline).unwrap();

    update_metadata(music_metadata).unwrap();

    thread::sleep(Duration::from_secs(10));
}
