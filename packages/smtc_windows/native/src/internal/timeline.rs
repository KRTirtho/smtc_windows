use std::time::Duration;

use windows::{Foundation::TimeSpan, Media::SystemMediaTransportControlsTimelineProperties};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct PlaybackTimeline {
    pub start_time_ms: i64,
    pub end_time_ms: i64,
    pub position_ms: i64,
    pub min_seek_time_ms: Option<i64>,
    pub max_seek_time_ms: Option<i64>,
}

impl PlaybackTimeline {
    pub fn t_start_time(&self) -> TimeSpan {
        TimeSpan::from(Duration::from_millis(self.start_time_ms as u64))
    }

    pub fn t_end_time(&self) -> TimeSpan {
        TimeSpan::from(Duration::from_millis(self.end_time_ms as u64))
    }

    pub fn t_position(&self) -> TimeSpan {
        TimeSpan::from(Duration::from_millis(self.position_ms as u64))
    }

    pub fn t_min_seek_time(&self) -> Option<TimeSpan> {
        self.min_seek_time_ms
            .map(|d| TimeSpan::from(Duration::from_millis(d as u64)))
    }

    pub fn t_max_seek_time(&self) -> Option<TimeSpan> {
        self.max_seek_time_ms
            .map(|d| TimeSpan::from(Duration::from_millis(d as u64)))
    }
}

impl Into<anyhow::Result<SystemMediaTransportControlsTimelineProperties>> for PlaybackTimeline {
    fn into(self) -> anyhow::Result<SystemMediaTransportControlsTimelineProperties> {
        let timeline = SystemMediaTransportControlsTimelineProperties::new()?;

        timeline.SetStartTime(self.t_start_time())?;
        timeline.SetEndTime(self.t_end_time())?;
        timeline.SetPosition(self.t_position())?;

        if let Some(min_seek_time) = self.t_min_seek_time() {
            timeline.SetMinSeekTime(min_seek_time)?;
        }

        if let Some(max_seek_time) = self.t_max_seek_time() {
            timeline.SetMaxSeekTime(max_seek_time)?;
        }

        return Ok(timeline);
    }
}
