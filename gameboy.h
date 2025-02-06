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

typedef struct String String;

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

struct ImageBuffer image(void);

extern void log(struct String s);

extern void log_u32(uint32_t a);
