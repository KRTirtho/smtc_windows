import 'dart:async';

import 'package:smtc_windows/src/bridge_generated.dart';
import 'package:smtc_windows/src/enums/button_event.dart';
import 'package:smtc_windows/src/enums/repeat_mode.dart';
import 'package:smtc_windows/src/ffi.dart';
import 'package:smtc_windows/src/extensions.dart';

final SmtcWindows _api = createLib();
SMTCWindows? _instance;

class SMTCWindows {
  SMTCWindows._({
    required SMTCConfig config,
    required PlaybackTimeline timeline,
  })  : _config = config,
        _timeline = timeline,
        _status = PlaybackStatus.Playing,
        _metadata = const MusicMetadata(
          title: '',
          album: '',
          albumArtist: '',
          artist: '',
          thumbnail: '',
          trackNumber: 0,
        );

  SMTCConfig _config;
  PlaybackTimeline _timeline;
  MusicMetadata _metadata;
  PlaybackStatus _status;

  Stream<PressedButton>? _buttonPressedStream;
  Stream<bool>? _shuffleChangeStream;
  Stream<RepeatMode>? _repeatModeChangeStream;

  bool _shuffleEnabled = false;
  RepeatMode _repeatMode = RepeatMode.none;

  factory SMTCWindows() {
    if (_instance == null) {
      throw StateError('SMTCWindows has not been initialized');
    }

    return _instance!;
  }

  static SMTCWindows get instance => SMTCWindows();

  static SMTCConfig get config => instance._config;
  static PlaybackTimeline get timeline => instance._timeline;
  static MusicMetadata get metadata => instance._metadata;
  static PlaybackStatus get status => instance._status;
  static Stream<PressedButton> get buttonPressStream =>
      instance._buttonPressedStream!;
  static Stream<bool> get shuffleChangeStream => instance._shuffleChangeStream!;
  static Stream<RepeatMode> get repeatModeChangeStream =>
      instance._repeatModeChangeStream!;

  static bool get isPlayEnabled => config.playEnabled;
  static bool get isPauseEnabled => config.pauseEnabled;
  static bool get isStopEnabled => config.stopEnabled;
  static bool get isNextEnabled => config.nextEnabled;
  static bool get isPrevEnabled => config.prevEnabled;
  static bool get isFastForwardEnabled => config.fastForwardEnabled;
  static bool get isRewindEnabled => config.rewindEnabled;

  static bool get isShuffleEnabled => instance._shuffleEnabled;
  static RepeatMode get repeatMode => instance._repeatMode;

  static Duration get startTime => Duration(milliseconds: timeline.startTimeMs);
  static Duration get endTime => Duration(milliseconds: timeline.endTimeMs);
  static Duration get position => Duration(milliseconds: timeline.positionMs);
  static Duration? get minSeekTime => timeline.minSeekTimeMs == null
      ? null
      : Duration(milliseconds: timeline.minSeekTimeMs!);
  static Duration? get maxSeekTime => timeline.maxSeekTimeMs == null
      ? null
      : Duration(milliseconds: timeline.maxSeekTimeMs!);

  static Future<void> initialize({
    required PlaybackTimeline timeline,
    SMTCConfig config = const SMTCConfig(
      playEnabled: true,
      pauseEnabled: true,
      nextEnabled: true,
      prevEnabled: true,
      stopEnabled: false,
      fastForwardEnabled: false,
      rewindEnabled: false,
    ),
  }) async {
    _instance ??= SMTCWindows._(config: config, timeline: timeline);
    await _api.initializeMediaPlayer(config: config, timeline: timeline);
    instance._buttonPressedStream ??= _api
        .buttonPressEvent()
        .map((event) => PressedButton.fromString(event))
        .asBroadcastStream();
    instance._shuffleChangeStream ??=
        _api.shuffleRequestEvent().asBroadcastStream();
    instance._repeatModeChangeStream ??= _api
        .repeatModeRequestEvent()
        .map(RepeatMode.fromString)
        .asBroadcastStream();
  }

  static Future<void> updateConfig(SMTCConfig config) {
    instance._config = config;
    return _api.updateConfig(config: config);
  }

  static Future<void> updateTimeline(PlaybackTimeline timeline) {
    instance._timeline = timeline;
    return _api.updateTimeline(timeline: timeline);
  }

  static Future<void> updateMetadata(MusicMetadata metadata) {
    instance._metadata = metadata;
    return _api.updateMetadata(metadata: metadata);
  }

  static Future<void> disable() async {
    await _api.updatePlaybackStatus(status: PlaybackStatus.Closed);
    await _api.disableSmtc();
    _instance = null;
  }

  static Future<void> setPlaybackStatus(PlaybackStatus status) {
    instance._status = status;
    return _api.updatePlaybackStatus(status: status);
  }

  static Future<void> setIsPlayEnabled(bool enabled) {
    return updateConfig(config.copyWith(playEnabled: enabled));
  }

  static Future<void> setIsPauseEnabled(bool enabled) {
    return updateConfig(config.copyWith(pauseEnabled: enabled));
  }

  static Future<void> setIsStopEnabled(bool enabled) {
    return updateConfig(config.copyWith(stopEnabled: enabled));
  }

  static Future<void> setIsNextEnabled(bool enabled) {
    return updateConfig(config.copyWith(nextEnabled: enabled));
  }

  static Future<void> setIsPrevEnabled(bool enabled) {
    return updateConfig(config.copyWith(prevEnabled: enabled));
  }

  static Future<void> setIsFastForwardEnabled(bool enabled) {
    return updateConfig(config.copyWith(fastForwardEnabled: enabled));
  }

  static Future<void> setIsRewindEnabled(bool enabled) {
    return updateConfig(config.copyWith(rewindEnabled: enabled));
  }

  static Future<void> setTimeline(PlaybackTimeline timeline) {
    return updateTimeline(timeline);
  }

  static Future<void> setTitle(String title) {
    return updateMetadata(metadata.copyWith(title: title));
  }

  static Future<void> setArtist(String artist) {
    return updateMetadata(metadata.copyWith(artist: artist));
  }

  static Future<void> setAlbum(String album) {
    return updateMetadata(metadata.copyWith(album: album));
  }

  static Future<void> setAlbumArtist(String albumArtist) {
    return updateMetadata(metadata.copyWith(albumArtist: albumArtist));
  }

  static Future<void> setThumbnail(String thumbnail) {
    return updateMetadata(metadata.copyWith(thumbnail: thumbnail));
  }

  static Future<void> setTrackNumber(int trackNumber) {
    return updateMetadata(metadata.copyWith(trackNumber: trackNumber));
  }

  static Future<void> setPosition(Duration position) {
    return updateTimeline(
      timeline.copyWith(positionMs: position.inMilliseconds),
    );
  }

  static Future<void> setStartTime(Duration startTime) {
    return updateTimeline(
      timeline.copyWith(startTimeMs: startTime.inMilliseconds),
    );
  }

  static Future<void> setEndTime(Duration endTime) {
    return updateTimeline(
      timeline.copyWith(endTimeMs: endTime.inMilliseconds),
    );
  }

  static Future<void> setMaxSeekTime(Duration maxSeekTime) {
    return updateTimeline(
      timeline.copyWith(maxSeekTimeMs: maxSeekTime.inMilliseconds),
    );
  }

  static Future<void> setMinSeekTime(Duration minSeekTime) {
    return updateTimeline(
      timeline.copyWith(minSeekTimeMs: minSeekTime.inMilliseconds),
    );
  }

  static Future<void> setShuffleEnabled(bool enabled) {
    instance._shuffleEnabled = enabled;
    return _api.updateShuffle(shuffle: enabled);
  }

  static Future<void> setRepeatMode(RepeatMode repeatMode) {
    instance._repeatMode = repeatMode;
    return _api.updateRepeatMode(repeatMode: repeatMode.asString);
  }
}
