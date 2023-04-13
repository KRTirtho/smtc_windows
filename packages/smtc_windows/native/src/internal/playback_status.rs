use windows::Media::MediaPlaybackStatus;

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
