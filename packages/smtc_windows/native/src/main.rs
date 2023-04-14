use std::{thread, time::Duration};

pub mod api;
pub mod internal;

use crate::{
    api::{smtc_new, smtc_update_config, smtc_update_metadata, smtc_update_timeline, SMTCInternal},
    internal::config::SMTCConfig,
    internal::metadata::MusicMetadata,
    internal::timeline::PlaybackTimeline,
};

fn main() -> anyhow::Result<()> {
    let smtc_config = SMTCConfig::default();
    let timeline = PlaybackTimeline {
        start_time_ms: 0,
        end_time_ms: 1000,
        position_ms: 0,
        max_seek_time_ms: None,
        min_seek_time_ms: None,
    };
    let music_metadata = MusicMetadata{
      album: Some("Bangerz".to_string()),
      album_artist: Some("Miley Cyrus".to_string()),
      artist: Some("Miley Cyrus".to_string()),
      title: Some("Wrecking Ball".to_string()),
      thumbnail: Some("https://media.glamour.com/photos/5f4c44e20c71c58fc210d35f/master/w_2560%2Cc_limit/mgid_ao_image_mtv.jpg".to_string()),
    };

    println!("Waiting 10 seconds...");

    let smtc_internal = smtc_new(None)?.0;

    smtc_update_config(smtc_internal.clone(), smtc_config)?;
    smtc_update_metadata(smtc_internal.clone(), music_metadata)?;
    smtc_update_timeline(smtc_internal.clone(), timeline)?;

    thread::sleep(Duration::from_secs(10));

    drop(smtc_internal);

    println!("Now with the new API");

    let smtc_internal = SMTCInternal::new(None)?;

    let music_metadata=  MusicMetadata{
      album: Some("Younger Now".to_string()),
      album_artist: Some("Miley Cyrus".to_string()),
      artist: Some("Miley Cyrus".to_string()),
      title: Some("Malibu".to_string()),
      thumbnail: Some("https://upload.wikimedia.org/wikipedia/en/thumb/7/72/Miley_Cyrus_-_Younger_Now_%28Official_Album_Cover%29.png/220px-Miley_Cyrus_-_Younger_Now_%28Official_Album_Cover%29.png".to_string()),
    };

    smtc_internal.update_config(smtc_config)?;
    smtc_internal.update_metadata(music_metadata)?;
    smtc_internal.update_timeline(timeline)?;

    println!("Waiting 10 seconds...");

    thread::sleep(Duration::from_secs(10));

    smtc_internal.clear_metadata()?;
    smtc_internal.disable_smtc()?;

    thread::sleep(Duration::from_secs(4));

    println!("Done");

    Ok(())
}
