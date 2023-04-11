#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_SMTCConfig {
  bool play_enabled;
  bool pause_enabled;
  bool stop_enabled;
  bool next_enabled;
  bool prev_enabled;
  bool fast_forward_enabled;
  bool rewind_enabled;
} wire_SMTCConfig;

typedef struct wire_PlaybackTimeline {
  int64_t start_time_ms;
  int64_t end_time_ms;
  int64_t position_ms;
  int64_t *min_seek_time_ms;
  int64_t *max_seek_time_ms;
} wire_PlaybackTimeline;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_MusicMetadata {
  struct wire_uint_8_list *title;
  struct wire_uint_8_list *artist;
  struct wire_uint_8_list *album;
  struct wire_uint_8_list *album_artist;
  uint32_t track_number;
  struct wire_uint_8_list *thumbnail;
} wire_MusicMetadata;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_initialize_media_player(int64_t port_,
                                  struct wire_SMTCConfig *config,
                                  struct wire_PlaybackTimeline *timeline);

void wire_update_config(int64_t port_, struct wire_SMTCConfig *config);

void wire_update_metadata(int64_t port_, struct wire_MusicMetadata *metadata);

void wire_update_timeline(int64_t port_, struct wire_PlaybackTimeline *timeline);

void wire_update_playback_status(int64_t port_, int32_t status);

void wire_disable_smtc(int64_t port_);

int64_t *new_box_autoadd_i64_0(int64_t value);

struct wire_MusicMetadata *new_box_autoadd_music_metadata_0(void);

struct wire_PlaybackTimeline *new_box_autoadd_playback_timeline_0(void);

struct wire_SMTCConfig *new_box_autoadd_smtc_config_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_initialize_media_player);
    dummy_var ^= ((int64_t) (void*) wire_update_config);
    dummy_var ^= ((int64_t) (void*) wire_update_metadata);
    dummy_var ^= ((int64_t) (void*) wire_update_timeline);
    dummy_var ^= ((int64_t) (void*) wire_update_playback_status);
    dummy_var ^= ((int64_t) (void*) wire_disable_smtc);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_music_metadata_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_playback_timeline_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_smtc_config_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
