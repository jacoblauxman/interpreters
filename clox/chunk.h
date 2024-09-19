#ifndef clox_chunk_h
#define clox_chunk_h

#include "common.h"
#include "value.h"

// controls what instruction work being dealt with
typedef enum {
    OP_CONSTANT,
    OP_RETURN,
} OpCode;

// for data storage (alongside instructions) (wrapper around array of bytes for now) -- ryo Dynamic Array (ArrayList)
typedef struct {
    int count;
    int capacity;
    uint8_t* code;
    int* lines; // each int in arr is line # for corresponding byte in bytecode
    ValueArray constants;
} Chunk;

void initChunk(Chunk* chunk);
void freeChunk(Chunk* chunk);
void writeChunk(Chunk* chunk, uint8_t byte, int line);
int addConstant(Chunk* chunk, Value value);

#endif

// notes: `lines` is kept separate since info only useful with RTE, don't put between instructions to take up CPU cache (cache misses++)
