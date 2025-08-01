package main

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is sometimes necessary to fix linker errors about `dlsym`.

/*
#cgo LDFLAGS: ./libgameboy.a -ldl
#include "../../gameboy.h"
#include <stdlib.h>
*/
import "C"

func main() {
	if len(os.Args) <= 1 {
		fmt.Fprintf(os.Stderr, "Please provide the game path\ne.g: $ boyband my-game.gb")
		os.Exit(0)
	}

	romPath := os.Args[1]
	romData, romErr := loadROMData(romPath)
	if romErr != nil {
		fmt.Fprintf(os.Stderr, "Error while loading the rom file: %s", romErr)
		os.Exit(1)
	}

	C.load((*C.uchar)(&romData[0]), C.size_t(len(romData)))
}
