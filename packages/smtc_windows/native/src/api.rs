use flutter_rust_bridge::{RustOpaque, StreamSink, SyncReturn};
use lazy_static::lazy_static;

use windows::{
    Foundation::{self, TypedEventHandler},
    Media::{
        AutoRepeatModeChangeRequestedEventArgs, MediaPlaybackAutoRepeatMode, MediaPlaybackStatus,
        MediaPlaybackType, PlaybackPositionChangeRequestedEventArgs,
        ShuffleEnabledChangeRequestedEventArgs, SystemMediaTransportControls,
        SystemMediaTransportControlsButton, SystemMediaTransportControlsButtonPressedEventArgs,
        SystemMediaTransportControlsTimelineProperties,
    },
    Storage::Streams::RandomAccessStreamReference,
};

use super::{config::SMTCConfig, metadata::MusicMetadata, timeline::PlaybackTimeline};

lazy_static! {
    static ref MEDIA_PLAYER: std::sync::Mutex<windows::Media::Playback::MediaPlayer> =
        std::sync::Mutex::new(windows::Media::Playback::MediaPlayer::new().unwrap());
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

pub fn smtc_new(
) -> anyhow::Result<SyncReturn<RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>>>
{
    Ok(SyncReturn(RustOpaque::new(std::sync::Mutex::new(
        windows::Media::Playback::MediaPlayer::new()?,
    ))))
}

pub fn smtc_update_config(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
    config: SMTCConfig,
) -> anyhow::Result<()> {
    let media_player = media_player.lock().unwrap();
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

pub fn smtc_update_metadata(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
    metadata: MusicMetadata,
) -> anyhow::Result<()> {
    let media_player = media_player.lock().unwrap();
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

pub fn smtc_update_timeline(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
    timeline: PlaybackTimeline,
) -> anyhow::Result<()> {
    let media_player = media_player.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    let timeline_properties: anyhow::Result<SystemMediaTransportControlsTimelineProperties> =
        timeline.into();
    smtc.UpdateTimelineProperties(&timeline_properties?)?;

    Ok(())
}

pub fn smtc_update_playback_status(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
    status: PlaybackStatus,
) -> anyhow::Result<()> {
    let media_player = media_player.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;
    smtc.SetPlaybackStatus(status.into())?;
    Ok(())
}

pub fn smtc_update_shuffle(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
    shuffle: bool,
) -> anyhow::Result<()> {
    let media_player = media_player.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;
    smtc.SetShuffleEnabled(shuffle)?;
    Ok(())
}

pub fn smtc_update_repeat_mode(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
    repeat_mode: String,
) -> anyhow::Result<()> {
    let media_player = media_player.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    match repeat_mode.as_str() {
        "none" => smtc.SetAutoRepeatMode(MediaPlaybackAutoRepeatMode::None)?,
        "track" => smtc.SetAutoRepeatMode(MediaPlaybackAutoRepeatMode::Track)?,
        "list" => smtc.SetAutoRepeatMode(MediaPlaybackAutoRepeatMode::List)?,
        _ => smtc.SetAutoRepeatMode(MediaPlaybackAutoRepeatMode::None)?,
    }

    Ok(())
}

pub fn smtc_disable_smtc(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
) -> anyhow::Result<()> {
    let media_player = media_player.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls();
    smtc?.SetIsEnabled(false)?;
    Ok(())
}

pub fn smtc_button_press_event(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
    sink: StreamSink<String>,
) -> anyhow::Result<()> {
    let handler = TypedEventHandler::<
        SystemMediaTransportControls,
        SystemMediaTransportControlsButtonPressedEventArgs,
    >::new(move |_, args| {
        let button = args.as_ref().unwrap().Button().unwrap();

        match button {
            SystemMediaTransportControlsButton::Play => {
                sink.add("play".to_string());
            }
            SystemMediaTransportControlsButton::Pause => {
                sink.add("pause".to_string());
            }
            SystemMediaTransportControlsButton::Next => {
                sink.add("next".to_string());
            }
            SystemMediaTransportControlsButton::Previous => {
                sink.add("previous".to_string());
            }
            SystemMediaTransportControlsButton::FastForward => {
                sink.add("fast_forward".to_string());
            }
            SystemMediaTransportControlsButton::Rewind => {
                sink.add("rewind".to_string());
            }
            SystemMediaTransportControlsButton::Stop => {
                sink.add("stop".to_string());
            }
            SystemMediaTransportControlsButton::Record => {
                sink.add("record".to_string());
            }
            SystemMediaTransportControlsButton::ChannelUp => {
                sink.add("channel_up".to_string());
            }
            SystemMediaTransportControlsButton::ChannelDown => {
                sink.add("channel_down".to_string());
            }
            _ => {}
        }
        Ok(())
    });

    let media_player = media_player.lock().unwrap();

    let smtc = media_player.SystemMediaTransportControls()?;

    smtc.ButtonPressed(&handler)?;

    anyhow::Result::Ok(())
}

pub fn smtc_position_change_request_event(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
    sink: StreamSink<i64>,
) -> anyhow::Result<()> {
    let handler = TypedEventHandler::<
        SystemMediaTransportControls,
        PlaybackPositionChangeRequestedEventArgs,
    >::new(move |_, args| {
        let position_ms = args
            .as_ref()
            .unwrap()
            .RequestedPlaybackPosition()
            .unwrap()
            .Duration;

        sink.add(position_ms);
        Ok(())
    });

    let media_player = media_player.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    smtc.PlaybackPositionChangeRequested(&handler)?;

    anyhow::Result::Ok(())
}

pub fn smtc_shuffle_request_event(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
    sink: StreamSink<bool>,
) -> anyhow::Result<()> {
    let handler = TypedEventHandler::<
        SystemMediaTransportControls,
        ShuffleEnabledChangeRequestedEventArgs,
    >::new(move |_, args| {
        let shuffle = args.as_ref().unwrap().RequestedShuffleEnabled().unwrap();

        sink.add(shuffle);
        Ok(())
    });

    let media_player = media_player.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    smtc.ShuffleEnabledChangeRequested(&handler)?;

    anyhow::Result::Ok(())
}

pub fn smtc_repeat_mode_request_event(
    media_player: RustOpaque<std::sync::Mutex<windows::Media::Playback::MediaPlayer>>,
    sink: StreamSink<String>,
) -> anyhow::Result<()> {
    let handler = TypedEventHandler::<
        SystemMediaTransportControls,
        AutoRepeatModeChangeRequestedEventArgs,
    >::new(move |_, args| {
        let repeat_mode = args.as_ref().unwrap().RequestedAutoRepeatMode().unwrap();

        match repeat_mode {
            MediaPlaybackAutoRepeatMode::None => {
                sink.add("none".to_string());
            }
            MediaPlaybackAutoRepeatMode::Track => {
                sink.add("track".to_string());
            }
            MediaPlaybackAutoRepeatMode::List => {
                sink.add("list".to_string());
            }
            _ => {
                sink.add("none".to_string());
            }
        }

        Ok(())
    });

    let media_player = media_player.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    smtc.AutoRepeatModeChangeRequested(&handler)?;

    anyhow::Result::Ok(())
}

// old

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

pub fn update_shuffle(shuffle: bool) -> anyhow::Result<()> {
    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;
    smtc.SetShuffleEnabled(shuffle)?;
    Ok(())
}

pub fn update_repeat_mode(repeat_mode: String) -> anyhow::Result<()> {
    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    match repeat_mode.as_str() {
        "none" => smtc.SetAutoRepeatMode(MediaPlaybackAutoRepeatMode::None)?,
        "track" => smtc.SetAutoRepeatMode(MediaPlaybackAutoRepeatMode::Track)?,
        "list" => smtc.SetAutoRepeatMode(MediaPlaybackAutoRepeatMode::List)?,
        _ => smtc.SetAutoRepeatMode(MediaPlaybackAutoRepeatMode::None)?,
    }

    Ok(())
}

pub fn disable_smtc() -> anyhow::Result<()> {
    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls();
    smtc?.SetIsEnabled(false)?;
    Ok(())
}

pub fn button_press_event(sink: StreamSink<String>) -> anyhow::Result<()> {
    let handler = TypedEventHandler::<
        SystemMediaTransportControls,
        SystemMediaTransportControlsButtonPressedEventArgs,
    >::new(move |_, args| {
        let button = args.as_ref().unwrap().Button().unwrap();

        match button {
            SystemMediaTransportControlsButton::Play => {
                sink.add("play".to_string());
            }
            SystemMediaTransportControlsButton::Pause => {
                sink.add("pause".to_string());
            }
            SystemMediaTransportControlsButton::Next => {
                sink.add("next".to_string());
            }
            SystemMediaTransportControlsButton::Previous => {
                sink.add("previous".to_string());
            }
            SystemMediaTransportControlsButton::FastForward => {
                sink.add("fast_forward".to_string());
            }
            SystemMediaTransportControlsButton::Rewind => {
                sink.add("rewind".to_string());
            }
            SystemMediaTransportControlsButton::Stop => {
                sink.add("stop".to_string());
            }
            SystemMediaTransportControlsButton::Record => {
                sink.add("record".to_string());
            }
            SystemMediaTransportControlsButton::ChannelUp => {
                sink.add("channel_up".to_string());
            }
            SystemMediaTransportControlsButton::ChannelDown => {
                sink.add("channel_down".to_string());
            }
            _ => {}
        }
        Ok(())
    });

    let media_player = MEDIA_PLAYER.lock().unwrap();

    let smtc = media_player.SystemMediaTransportControls()?;

    smtc.ButtonPressed(&handler)?;

    anyhow::Result::Ok(())
}

pub fn position_change_request_event(sink: StreamSink<i64>) -> anyhow::Result<()> {
    let handler = TypedEventHandler::<
        SystemMediaTransportControls,
        PlaybackPositionChangeRequestedEventArgs,
    >::new(move |_, args| {
        let position_ms = args
            .as_ref()
            .unwrap()
            .RequestedPlaybackPosition()
            .unwrap()
            .Duration;

        sink.add(position_ms);
        Ok(())
    });

    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    smtc.PlaybackPositionChangeRequested(&handler)?;

    anyhow::Result::Ok(())
}

pub fn shuffle_request_event(sink: StreamSink<bool>) -> anyhow::Result<()> {
    let handler = TypedEventHandler::<
        SystemMediaTransportControls,
        ShuffleEnabledChangeRequestedEventArgs,
    >::new(move |_, args| {
        let shuffle = args.as_ref().unwrap().RequestedShuffleEnabled().unwrap();

        sink.add(shuffle);
        Ok(())
    });

    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    smtc.ShuffleEnabledChangeRequested(&handler)?;

    anyhow::Result::Ok(())
}

pub fn repeat_mode_request_event(sink: StreamSink<String>) -> anyhow::Result<()> {
    let handler = TypedEventHandler::<
        SystemMediaTransportControls,
        AutoRepeatModeChangeRequestedEventArgs,
    >::new(move |_, args| {
        let repeat_mode = args.as_ref().unwrap().RequestedAutoRepeatMode().unwrap();

        match repeat_mode {
            MediaPlaybackAutoRepeatMode::None => {
                sink.add("none".to_string());
            }
            MediaPlaybackAutoRepeatMode::Track => {
                sink.add("track".to_string());
            }
            MediaPlaybackAutoRepeatMode::List => {
                sink.add("list".to_string());
            }
            _ => {
                sink.add("none".to_string());
            }
        }

        Ok(())
    });

    let media_player = MEDIA_PLAYER.lock().unwrap();
    let smtc = media_player.SystemMediaTransportControls()?;

    smtc.AutoRepeatModeChangeRequested(&handler)?;

    anyhow::Result::Ok(())
}
