import 'dart:async';

import 'package:smtc_windows/src/bridge_generated.dart';
import 'package:smtc_windows/src/enums/button_event.dart';
import 'package:smtc_windows/src/enums/repeat_mode.dart';
import 'package:smtc_windows/src/ffi.dart';
import 'package:smtc_windows/src/extensions.dart';

final SmtcWindows api = createLib();

class SMTCWindows {
  //! Unsafe shared pointer to the underlying Rust struct.
  late final SmtcInternal _internal;

  SMTCConfig _config;
  PlaybackTimeline _timeline;
  MusicMetadata _metadata;
  PlaybackStatus? _status;

  late final Stream<PressedButton> _buttonPressedStream;
  late final Stream<bool> _shuffleChangeStream;
  late final Stream<RepeatMode> _repeatModeChangeStream;

  late bool _shuffleEnabled;
  late RepeatMode _repeatMode;

  SMTCWindows({
    SMTCConfig? config,
    PlaybackTimeline? timeline,
    MusicMetadata? metadata,
    PlaybackStatus? status,
    bool? shuffleEnabled,
    RepeatMode? repeatMode,
  })  : _internal = api.smtcNew(),
        _status = status,
        _config = config ??
            const SMTCConfig(
              playEnabled: true,
              pauseEnabled: true,
              nextEnabled: true,
              prevEnabled: true,
              stopEnabled: false,
              fastForwardEnabled: false,
              rewindEnabled: false,
            ),
        _timeline = timeline ??
            const PlaybackTimeline(
              positionMs: 0,
              startTimeMs: 0,
              endTimeMs: 0,
            ),
        _metadata = metadata ??
            const MusicMetadata(
              title: '',
              album: '',
              albumArtist: '',
              artist: '',
              thumbnail: '',
              trackNumber: 0,
            ),
        _shuffleEnabled = shuffleEnabled ?? false,
        _repeatMode = repeatMode ?? RepeatMode.none {
    _buttonPressedStream = api
        .smtcButtonPressEvent(internal: _internal)
        .map((event) => PressedButton.fromString(event))
        .asBroadcastStream();
    _shuffleChangeStream =
        api.smtcShuffleRequestEvent(internal: _internal).asBroadcastStream();
    _repeatModeChangeStream = api
        .smtcRepeatModeRequestEvent(internal: _internal)
        .map(RepeatMode.fromString)
        .asBroadcastStream();

    updateConfig(_config);

    if (status != null) {
      setPlaybackStatus(status);
    }
    if (timeline != null) {
      updateTimeline(timeline);
    }
    if (metadata != null) {
      updateMetadata(metadata);
    }
    if (shuffleEnabled != null) {
      setShuffleEnabled(shuffleEnabled);
    }
    if (repeatMode != null) {
      setRepeatMode(repeatMode);
    }
  }

  SMTCConfig get config => _config;
  PlaybackTimeline get timeline => _timeline;
  MusicMetadata get metadata => _metadata;
  PlaybackStatus? get status => _status;
  Stream<PressedButton> get buttonPressStream => _buttonPressedStream;
  Stream<bool> get shuffleChangeStream => _shuffleChangeStream;
  Stream<RepeatMode> get repeatModeChangeStream => _repeatModeChangeStream;

  bool get isPlayEnabled => config.playEnabled;
  bool get isPauseEnabled => config.pauseEnabled;
  bool get isStopEnabled => config.stopEnabled;
  bool get isNextEnabled => config.nextEnabled;
  bool get isPrevEnabled => config.prevEnabled;
  bool get isFastForwardEnabled => config.fastForwardEnabled;
  bool get isRewindEnabled => config.rewindEnabled;

  bool get isShuffleEnabled => _shuffleEnabled;
  RepeatMode get repeatMode => _repeatMode;

  Duration? get startTime => Duration(milliseconds: timeline.startTimeMs);
  Duration? get endTime => Duration(milliseconds: timeline.endTimeMs);
  Duration? get position => Duration(milliseconds: timeline.positionMs);
  Duration? get minSeekTime => timeline.minSeekTimeMs == null
      ? null
      : Duration(milliseconds: timeline.minSeekTimeMs!);
  Duration? get maxSeekTime => timeline.maxSeekTimeMs == null
      ? null
      : Duration(milliseconds: timeline.maxSeekTimeMs!);

  Future<void> updateConfig(SMTCConfig config) {
    _config = config;
    return api.smtcUpdateConfig(
      internal: _internal,
      config: config,
    );
  }

  Future<void> updateTimeline(PlaybackTimeline timeline) {
    _timeline = timeline;
    return api.smtcUpdateTimeline(
      internal: _internal,
      timeline: timeline,
    );
  }

  Future<void> updateMetadata(MusicMetadata metadata) {
    _metadata = metadata;
    return api.smtcUpdateMetadata(
      internal: _internal,
      metadata: metadata,
    );
  }

  Future<void> dispose() async {
    await api.smtcUpdatePlaybackStatus(
      internal: _internal,
      status: PlaybackStatus.Closed,
    );
    await api.smtcDisableSmtc(internal: _internal);
    _internal.dispose();
  }

  Future<void> setPlaybackStatus(PlaybackStatus status) async {
    _status = status;
    return api.smtcUpdatePlaybackStatus(
      internal: _internal,
      status: status,
    );
  }

  Future<void> setIsPlayEnabled(bool enabled) {
    return updateConfig(config.copyWith(playEnabled: enabled));
  }

  Future<void> setIsPauseEnabled(bool enabled) {
    return updateConfig(config.copyWith(pauseEnabled: enabled));
  }

  Future<void> setIsStopEnabled(bool enabled) {
    return updateConfig(config.copyWith(stopEnabled: enabled));
  }

  Future<void> setIsNextEnabled(bool enabled) {
    return updateConfig(config.copyWith(nextEnabled: enabled));
  }

  Future<void> setIsPrevEnabled(bool enabled) {
    return updateConfig(config.copyWith(prevEnabled: enabled));
  }

  Future<void> setIsFastForwardEnabled(bool enabled) {
    return updateConfig(config.copyWith(fastForwardEnabled: enabled));
  }

  Future<void> setIsRewindEnabled(bool enabled) {
    return updateConfig(config.copyWith(rewindEnabled: enabled));
  }

  Future<void> setTimeline(PlaybackTimeline timeline) {
    return updateTimeline(timeline);
  }

  Future<void> setTitle(String title) {
    return updateMetadata(metadata.copyWith(title: title));
  }

  Future<void> setArtist(String artist) {
    return updateMetadata(metadata.copyWith(artist: artist));
  }

  Future<void> setAlbum(String album) {
    return updateMetadata(metadata.copyWith(album: album));
  }

  Future<void> setAlbumArtist(String albumArtist) {
    return updateMetadata(metadata.copyWith(albumArtist: albumArtist));
  }

  Future<void> setThumbnail(String thumbnail) {
    return updateMetadata(metadata.copyWith(thumbnail: thumbnail));
  }

  Future<void> setTrackNumber(int trackNumber) {
    return updateMetadata(metadata.copyWith(trackNumber: trackNumber));
  }

  Future<void> setPosition(Duration position) {
    return updateTimeline(
      timeline.copyWith(positionMs: position.inMilliseconds),
    );
  }

  Future<void> setStartTime(Duration startTime) {
    return updateTimeline(
      timeline.copyWith(startTimeMs: startTime.inMilliseconds),
    );
  }

  Future<void> setEndTime(Duration endTime) {
    return updateTimeline(
      timeline.copyWith(endTimeMs: endTime.inMilliseconds),
    );
  }

  Future<void> setMaxSeekTime(Duration maxSeekTime) {
    return updateTimeline(
      timeline.copyWith(maxSeekTimeMs: maxSeekTime.inMilliseconds),
    );
  }

  Future<void> setMinSeekTime(Duration minSeekTime) {
    return updateTimeline(
      timeline.copyWith(minSeekTimeMs: minSeekTime.inMilliseconds),
    );
  }

  Future<void> setShuffleEnabled(bool enabled) {
    _shuffleEnabled = enabled;
    return api.smtcUpdateShuffle(
      internal: _internal,
      shuffle: enabled,
    );
  }

  Future<void> setRepeatMode(RepeatMode repeatMode) {
    _repeatMode = repeatMode;
    return api.smtcUpdateRepeatMode(
      internal: _internal,
      repeatMode: repeatMode.asString,
    );
  }
}
