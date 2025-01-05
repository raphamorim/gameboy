#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define CYCLES 70224

#define HEIGHT 144

#define WIDTH 160

enum KeypadKey {
  Right,
  Left,
  Up,
  Down,
  A,
  B,
  Select,
  Start,
};
typedef uint8_t KeypadKey;

typedef struct String String;

typedef struct ImageBuffer {
  int32_t len;
  const uint8_t *data;
} ImageBuffer;

void load_rom(const unsigned char *bytes, uintptr_t bytes_length);

void frame(void);

void keydown(KeypadKey key);

void keyup(KeypadKey key);

struct ImageBuffer image(void);

extern void log(struct String s);

extern void log_u32(uint32_t a);
