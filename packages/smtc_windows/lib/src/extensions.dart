import 'package:smtc_windows/src/bridge_generated.dart';

extension SMTCConfigCopy on SMTCConfig {
  SMTCConfig copyWith({
    bool? playEnabled,
    bool? pauseEnabled,
    bool? stopEnabled,
    bool? nextEnabled,
    bool? prevEnabled,
    bool? fastForwardEnabled,
    bool? rewindEnabled,
  }) {
    return SMTCConfig(
      playEnabled: playEnabled ?? this.playEnabled,
      pauseEnabled: pauseEnabled ?? this.pauseEnabled,
      stopEnabled: stopEnabled ?? this.stopEnabled,
      nextEnabled: nextEnabled ?? this.nextEnabled,
      prevEnabled: prevEnabled ?? this.prevEnabled,
      fastForwardEnabled: fastForwardEnabled ?? this.fastForwardEnabled,
      rewindEnabled: rewindEnabled ?? this.rewindEnabled,
    );
  }
}

extension PlaybackTimelineCopy on PlaybackTimeline {
  PlaybackTimeline copyWith({
    int? startTimeMs,
    int? endTimeMs,
    int? positionMs,
    int? minSeekTimeMs,
    int? maxSeekTimeMs,
  }) {
    return PlaybackTimeline(
      startTimeMs: startTimeMs ?? this.startTimeMs,
      endTimeMs: endTimeMs ?? this.endTimeMs,
      positionMs: positionMs ?? this.positionMs,
      minSeekTimeMs: minSeekTimeMs ?? this.minSeekTimeMs,
      maxSeekTimeMs: maxSeekTimeMs ?? this.maxSeekTimeMs,
    );
  }
}

extension MusicMetadataCopy on MusicMetadata {
  MusicMetadata copyWith({
    String? title,
    String? artist,
    String? album,
    String? albumArtist,
    int? trackNumber,
    String? thumbnail,
  }) {
    return MusicMetadata(
      title: title ?? this.title,
      artist: artist ?? this.artist,
      album: album ?? this.album,
      albumArtist: albumArtist ?? this.albumArtist,
      trackNumber: trackNumber ?? this.trackNumber,
      thumbnail: thumbnail ?? this.thumbnail,
    );
  }
}
