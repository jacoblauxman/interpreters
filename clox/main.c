#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "vm.h"


static void repl() {
    char line[1024];
    for (;;) {
        printf("> ");

        // read + store line from stdin (buffer, size, stream)
        if (!fgets(line, sizeof(line), stdin)) {
            printf("\n");
            break;
        }

        interpret(line);
    }
}

// need to allocate big enough string to read entire file, initially don't know file size until read
// -> open file, seek to very end via `fseek`, `ftell` gives back how many bytes from SOF (start of file) ie. the file size
// -> rewind back to beginning, allocate string of file size and read as one batch
static char* readFile(const char* path) {
    // (filename, mode) - 'rb' = 'read binary'
    FILE* file = fopen(path, "rb");

    if (file == NULL) {
        fprintf(stderr, "Could not open file \"%s\".\n", path);
        exit(74);
    }

    // stream, offset (long), origin (reference point)
    fseek(file, 0L, SEEK_END);
    // curr file ptr pos
    size_t fileSize = ftell(file);
    rewind(file);

    char* buffer = (char*)malloc(fileSize + 1);

    if (buffer == NULL) {
        fprintf(stderr, "Not enough memory to read \"%s\".\n", path);
        exit(74);
    }

    // ptr (to data storage), size (of each ele), count, stream
    size_t bytesRead = fread(buffer, sizeof(char), fileSize, file);

    if (bytesRead < fileSize) {
        fprintf(stderr, "Could not read file \"%s\".", path);
        exit(74);
    }

    buffer[bytesRead] = '\0';

    fclose(file);
    return buffer;
}

// read file, execute string of Lox, based on res return exit code
static void runFile(const char* path) {
    char* source = readFile(path);
    InterpretResult result = interpret(source);
    // need to since `readFile` dyn. allocates and passes ownership to caller
    free(source);

    if (result == INTERPRET_COMPILE_ERROR) exit(65);
    if (result == INTERPRET_RUNTIME_ERROR) exit(70);
}

int main(int argc, const char* argv[]) {
    initVM();

    if (argc == 1) {
        repl();
    } else if (argc == 2) {
        runFile(argv[1]);
    } else {
        fprintf(stderr, "Usage: clox [path]\n");
        exit(64);
    }

   //  Chunk chunk;
   //  initChunk(&chunk);

   //  int constant = addConstant(&chunk, 1.2);
   //  writeChunk(&chunk, OP_CONSTANT, 123);
   //  writeChunk(&chunk, constant, 123);

   //  constant = addConstant(&chunk, 3.4);
   //  writeChunk(&chunk,  OP_CONSTANT, 123);
   //  writeChunk(&chunk, constant, 123);

   //  writeChunk(&chunk, OP_ADD, 123);

   // constant = addConstant(&chunk, 5.6);
   // writeChunk(&chunk, OP_CONSTANT, 123);
   // writeChunk(&chunk, constant, 123);

   // writeChunk(&chunk, OP_DIVIDE, 123);

   //  writeChunk(&chunk, OP_NEGATE, 123);
   //  writeChunk(&chunk, OP_RETURN, 123);

   //  disassembleChunk(&chunk, "test chunk");
   //  interpret(&chunk);
   //  freeVM();
   //  freeChunk(&chunk);

   freeVM();
    return 0;
}
