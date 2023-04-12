enum PressedButton {
  play,
  pause,
  next,
  previous,
  fastForward,
  rewind,
  stop,
  record,
  channelUp,
  channelDown;

  static PressedButton fromString(String button) {
    switch (button) {
      case 'play':
        return PressedButton.play;
      case 'pause':
        return PressedButton.pause;
      case 'next':
        return PressedButton.next;
      case 'previous':
        return PressedButton.previous;
      case 'fast_forward':
        return PressedButton.fastForward;
      case 'rewind':
        return PressedButton.rewind;
      case 'stop':
        return PressedButton.stop;
      case 'record':
        return PressedButton.record;
      case 'channel_up':
        return PressedButton.channelUp;
      case 'channel_down':
        return PressedButton.channelDown;
      default:
        throw Exception('Unknown button: $button');
    }
  }
}
