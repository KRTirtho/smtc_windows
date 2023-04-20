## SMTC_Windows

[![pub package](https://img.shields.io/pub/v/smtc_windows.svg)](https://pub.dev/packages/smtc_windows)


Windows `SystemMediaTransportControls` implementation for Flutter. This is the windows equivalent of android's [audio_session](https://pub.dev/packages/audio_session) and Linux's D-Bus MPRIS (Media Player Remote Interfacing Specification)

### Install

Add `smtc_windows` as a dependency in your pubspec.yaml file.
```bash
flutter pub add smtc_windows
```

### Support Development

<a href="https://www.buymeacoffee.com/krtirtho">
<img src="https://img.buymeacoffee.com/button-api/?text=Buy me a coffee&emoji=&slug=krtirtho&button_colour=FF5F5F&font_colour=ffffff&font_family=Inter&outline_colour=000000&coffee_colour=FFDD00" />
</a>
<br/>
<a href="https://patreon.com/krtirtho"><img src="https://user-images.githubusercontent.com/61944859/180249027-678b01b8-c336-451e-b147-6d84a5b9d0e7.png" width="250"/></a>

### Usage

```dart
class _MyAppState extends State<MyApp> {
  late SMTCWindows smtc;

  @override
  void initState() {
    // initialize SMTC
    smtc = SMTCWindows(
      metadata: const MusicMetadata(
        title: 'Title',
        album: 'Album',
        albumArtist: 'Album Artist',
        artist: 'Artist',
        thumbnail:
            'https://media.glamour.com/photos/5f4c44e20c71c58fc210d35f/master/w_2560%2Cc_limit/mgid_ao_image_mtv.jpg',
      ),
      // Timeline info for the OS media player
      timeline: const PlaybackTimeline(
        startTimeMs: 0,
        endTimeMs: 1000,
        positionMs: 0,
        minSeekTimeMs: 0,
        maxSeekTimeMs: 1000,
      ),
      // Which buttons to show in the OS media player
      config: const SMTCConfig(
        fastForwardEnabled: true,
        nextEnabled: true,
        pauseEnabled: true,
        playEnabled: true,
        rewindEnabled: true,
        prevEnabled: true,
        stopEnabled: true,
      ),
    );
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      try {
        // Listen to button events and update playback status accordingly
        smtc.buttonPressStream.listen((event) {
          switch (event) {
            case PressedButton.play:
              // Update playback status
              smtc.setPlaybackStatus(PlaybackStatus.Playing);
              break;
            case PressedButton.pause:
              smtc.setPlaybackStatus(PlaybackStatus.Paused);
              break;
            case PressedButton.next:
              print('Next');
              break;
            case PressedButton.previous:
              print('Previous');
              break;
            case PressedButton.stop:
              smtc.setPlaybackStatus(PlaybackStatus.Stopped);
              smtc.disableSmtc();
              break;
            default:
              break;
          }
        });
      } catch (e) {
        debugPrint("Error: $e");
      }
    });
    super.initState();
  }

  @override
  void dispose() {
    // Dispose SMTC
    smtc.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Native Packages'),
        ),
        body: ElevatedButton(
          child: const Text("Click"),
          onPressed: () {
            // Update player metadata
            smtc.updateMetadata(
              const MusicMetadata(
                title: 'Title',
                album: 'Album',
                albumArtist: 'Album Artist',
                artist: 'Artist',
                thumbnail:
                    'https://media.glamour.com/photos/5f4c44e20c71c58fc210d35f/master/w_2560%2Cc_limit/mgid_ao_image_mtv.jpg',
              ),
            );
          },
        ),
      ),
    );
  }
}
```

### License
[MIT](packages/smtc_windows/LICENSE)

### Author

Kingkor Roy Tirtho

[![Twitter Follow](https://img.shields.io/twitter/follow/krtirtho?style=social)](https://twitter.com/krtirtho)

[![GitHub followers](https://img.shields.io/github/followers/krtirtho?style=social)](https://github.com/KRTirtho)