use flutter_rust_bridge::{RustOpaque, StreamSink, SyncReturn};

use crate::internal::playback_status::PlaybackStatus;

use super::internal::{config::SMTCConfig, metadata::MusicMetadata, timeline::PlaybackTimeline};

pub type SMTCInternal = super::internal::smtc_internal::SMTCInternal;

pub fn smtc_new() -> anyhow::Result<SyncReturn<RustOpaque<SMTCInternal>>> {
    let internal = SMTCInternal::new()?;
    Ok(SyncReturn(RustOpaque::new(internal)))
}

pub fn smtc_update_config(
    internal: RustOpaque<SMTCInternal>,
    config: SMTCConfig,
) -> anyhow::Result<()> {
    internal.update_config(config)
}

pub fn smtc_update_metadata(
    internal: RustOpaque<SMTCInternal>,
    metadata: MusicMetadata,
) -> anyhow::Result<()> {
    internal.update_metadata(metadata)
}

pub fn smtc_update_timeline(
    internal: RustOpaque<SMTCInternal>,
    timeline: PlaybackTimeline,
) -> anyhow::Result<()> {
    internal.update_timeline(timeline)
}

pub fn smtc_update_playback_status(
    internal: RustOpaque<SMTCInternal>,
    status: PlaybackStatus,
) -> anyhow::Result<()> {
    internal.update_playback_status(status)
}

pub fn smtc_update_shuffle(
    internal: RustOpaque<SMTCInternal>,
    shuffle: bool,
) -> anyhow::Result<()> {
    internal.update_shuffle(shuffle)
}

pub fn smtc_update_repeat_mode(
    internal: RustOpaque<SMTCInternal>,
    repeat_mode: String,
) -> anyhow::Result<()> {
    internal.update_repeat_mode(repeat_mode)
}

pub fn smtc_disable_smtc(internal: RustOpaque<SMTCInternal>) -> anyhow::Result<()> {
    internal.disable_smtc()
}

pub fn smtc_button_press_event(
    internal: RustOpaque<SMTCInternal>,
    sink: StreamSink<String>,
) -> anyhow::Result<()> {
    internal.button_press_event(sink)
}

pub fn smtc_position_change_request_event(
    internal: RustOpaque<SMTCInternal>,
    sink: StreamSink<i64>,
) -> anyhow::Result<()> {
    internal.position_change_request_event(sink)
}

pub fn smtc_shuffle_request_event(
    internal: RustOpaque<SMTCInternal>,
    sink: StreamSink<bool>,
) -> anyhow::Result<()> {
    internal.shuffle_request_event(sink)
}

pub fn smtc_repeat_mode_request_event(
    internal: RustOpaque<SMTCInternal>,
    sink: StreamSink<String>,
) -> anyhow::Result<()> {
    internal.repeat_mode_request_event(sink)
}
