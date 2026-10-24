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
 * Calling it again switches games: the running game is dropped, which
 * writes its save, and sound (if enabled) moves to the new game. If the new
 * cartridge is rejected, the running game keeps going.
 *
 * Returns 0 on success, -1 for invalid arguments and -2 if the core rejected
 * the cartridge (see `last_error`).
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
 * Writes a save state of the running game to `path` (through a temporary
 * file, so a crash never leaves half a state behind).
 *
 * Returns 0 on success, -1 for invalid arguments or no game, and -2 on a
 * write error (see `last_error`).
 */
int32_t save_state(const char *path);

/**
 * Restores a save state written by `save_state`. If the file can't be read
 * or belongs to another game, the running game continues unchanged.
 *
 * Returns 0 on success, -1 for invalid arguments or no game, and -2 if the
 * state was rejected (see `last_error`).
 */
int32_t load_state(const char *path);

/**
 * Starts sound on the default output device. Call after loading a game, from
 * the thread that will keep running (usually the main thread). Sound stays
 * on across `load_with_save` game switches.
 *
 * Returns the output sample rate in Hz, or 0 if audio could not start.
 */
uint32_t enable_audio(void);

/**
 * Stereo frames produced but not yet played. Divide by the sample rate for
 * latency; hosts use it to pace `frame()` to the audio clock.
 */
uint32_t audio_queued(void);

/**
 * Silences sound without stopping emulation, e.g. while fast-forwarding.
 * Muting also drops what's already queued.
 */
void set_audio_muted(bool muted);

/**
 * CPU state for debuggers.
 */
typedef struct DebugRegisters {
  uint8_t a;
  uint8_t f;
  uint8_t b;
  uint8_t c;
  uint8_t d;
  uint8_t e;
  uint8_t h;
  uint8_t l;
  uint16_t sp;
  uint16_t pc;
  bool ime;
  bool halted;
  /**
   * The ROM bank mapped at 0x4000-0x7FFF.
   */
  uint16_t rom_bank;
} DebugRegisters;

/**
 * Like `frame`, but stops early when the CPU reaches one of `breakpoints`.
 * Returns true if it stopped at a breakpoint; calling again continues.
 */
bool debug_frame(const uint16_t *breakpoints, uintptr_t count);

/**
 * Executes a single instruction. Returns the clock cycles it took, or 0
 * if no game is loaded.
 */
uint32_t debug_step(void);

struct DebugRegisters debug_registers(void);

/**
 * Reads a byte as the CPU sees it, without side effects.
 */
uint8_t debug_peek(uint16_t address);

/**
 * Writes a byte as the CPU would.
 */
void debug_poke(uint16_t address, uint8_t value);

/**
 * Both VRAM banks, 0x4000 bytes, owned by the core.
 */
struct ImageBuffer debug_vram(void);

/**
 * Sprite attribute memory, 0xA0 bytes, owned by the core.
 */
struct ImageBuffer debug_oam(void);

struct ImageBuffer image(void);

const char *image_base64(void);
