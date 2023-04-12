enum RepeatMode {
  none,
  track,
  list;

  static RepeatMode fromString(String value) {
    switch (value) {
      case 'none':
        return none;
      case 'track':
        return track;
      case 'list':
        return list;
      default:
        throw Exception('Unknown repeat mode: $value');
    }
  }

  String get asString => toString().split('.').last;
}
