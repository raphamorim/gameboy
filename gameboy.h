#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define CYCLES 70224

#define HEIGHT 144

#define WIDTH 160

typedef enum KeypadKey {
  Right,
  Left,
  Up,
  Down,
  A,
  B,
  Select,
  Start,
} KeypadKey;

typedef struct ImageBuffer {
  int32_t len;
  const uint8_t *data;
} ImageBuffer;

/**
 * # Safety
 *
 * This function is not safe due to from_raw_parts.
 */
void load(const unsigned char *bytes, uintptr_t bytes_length);

void frame(void);

void keydown(enum KeypadKey key);

void keyup(enum KeypadKey key);

/**
 * The message for the most recent failed call, or null. Valid until the
 * next failure.
 */
const char *last_error(void);

/**
 * Like `load`, but battery-backed cartridges keep their RAM in a `.gbsave`
 * file next to `rom_path` (the path only names the save; the ROM is not
 * read from it). Pass null for no saves.
 *
 * Returns 0 on success, 1 if a game is already loaded, -1 for invalid
 * arguments and -2 if the core rejected the cartridge (see `last_error`).
 *
 * # Safety
 *
 * `bytes` must point to `bytes_length` readable bytes, and `rom_path` must be
 * null or a NUL-terminated string.
 */
int32_t load_with_save(const unsigned char *bytes, uintptr_t bytes_length, const char *rom_path);

/**
 * Writes battery-backed cartridge RAM to its save file now. Safe to call
 * often; cartridges without a battery do nothing.
 *
 * Returns 0 on success, -1 if no game is loaded and -2 on a write error
 * (see `last_error`).
 */
int32_t save(void);

/**
 * Starts sound on the default output device. Call after loading a game, from
 * the thread that will keep running (usually the main thread).
 *
 * Returns the output sample rate in Hz, or 0 if audio could not start.
 */
uint32_t enable_audio(void);

/**
 * Stereo frames produced but not yet played. Divide by the sample rate for
 * latency; hosts use it to pace `frame()` to the audio clock.
 */
uint32_t audio_queued(void);

struct ImageBuffer image(void);

const char *image_base64(void);
