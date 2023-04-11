use std::sync::Mutex;

use lazy_static::lazy_static;
use windows::{
    Foundation,
    Media::{
        MediaPlaybackStatus, MediaPlaybackType, Playback::MediaPlayer,
        SystemMediaTransportControlsTimelineProperties,
    },
    Storage::Streams::RandomAccessStreamReference,
};

use super::{config::SMTCConfig, metadata::MusicMetadata, timeline::PlaybackTimeline};


lazy_static! {
    static ref MEDIA_PLAYER: Mutex<MediaPlayer> = Mutex::new(MediaPlayer::new().unwrap());
}

#[allow(dead_code)]

#[derive(Clone, Copy, Debug)]
pub enum PlaybackStatus {
    Closed,
    Changing,
    Stopped,
    Playing,
    Paused,
}

impl Into<MediaPlaybackStatus> for PlaybackStatus {
    fn into(self) -> MediaPlaybackStatus {
        match self {
            PlaybackStatus::Closed => MediaPlaybackStatus::Closed,
            PlaybackStatus::Changing => MediaPlaybackStatus::Changing,
            PlaybackStatus::Stopped => MediaPlaybackStatus::Stopped,
            PlaybackStatus::Playing => MediaPlaybackStatus::Playing,
            PlaybackStatus::Paused => MediaPlaybackStatus::Paused,
        }
    }
}

pub fn initialize_media_player(
    config: SMTCConfig,
    timeline: PlaybackTimeline,
) -> anyhow::Result<()> {
    let media_player = MEDIA_PLAYER.lock().unwrap();

    let smtc = media_player.SystemMediaTransportControls()?;

    media_player.CommandManager()?.SetIsEnabled(false)?;

    smtc.SetIsEnabled(true)?;

    drop(media_player);

    update_config(config)?;
    update_timeline(timeline)?;
    return anyhow::Result::Ok(());
}

pub fn update_config(config: SMTCConfig) -> anyhow::Result<()> {
    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    smtc.SetIsPlayEnabled(config.play_enabled)?;
    smtc.SetIsPauseEnabled(config.pause_enabled)?;
    smtc.SetIsNextEnabled(config.next_enabled)?;
    smtc.SetIsPreviousEnabled(config.prev_enabled)?;
    smtc.SetIsFastForwardEnabled(config.fast_forward_enabled)?;
    smtc.SetIsRewindEnabled(config.rewind_enabled)?;
    smtc.SetIsStopEnabled(config.stop_enabled)?;

    Ok(())
}

pub fn update_metadata(metadata: MusicMetadata) -> anyhow::Result<()> {
    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;
    let updater = smtc.DisplayUpdater()?;

    updater.SetType(MediaPlaybackType::Music)?;

    let music_properties = updater.MusicProperties()?;

    music_properties.SetArtist(&metadata.h_artist())?;
    music_properties.SetAlbumTitle(&metadata.h_album())?;
    music_properties.SetTitle(&metadata.h_title())?;
    music_properties.SetTrackNumber(metadata.track_number)?;
    music_properties.SetAlbumArtist(&metadata.h_album_artist())?;

    let thumbnail = &metadata.h_thumbnail().map(|h| {
        let uri = Foundation::Uri::CreateUri(&h).unwrap();
        RandomAccessStreamReference::CreateFromUri(&uri).unwrap()
    });

    if let Some(thumbnail) = thumbnail {
        updater.SetThumbnail(thumbnail)?;
    }

    updater.Update()?;

    Ok(())
}

pub fn update_timeline(timeline: PlaybackTimeline) -> anyhow::Result<()> {
    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    let timeline_properties: anyhow::Result<SystemMediaTransportControlsTimelineProperties> =
        timeline.into();
    smtc.UpdateTimelineProperties(&timeline_properties?)?;

    Ok(())
}

pub fn update_playback_status(status: PlaybackStatus) -> anyhow::Result<()> {
    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;
    smtc.SetPlaybackStatus(status.into())?;
    Ok(())
}

pub fn disable_smtc() -> anyhow::Result<()> {
    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls();
    smtc?.SetIsEnabled(false)?;
    Ok(())
}
