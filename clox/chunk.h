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
    ValueArray constants;
} Chunk;

void initChunk(Chunk* chunk);
void freeChunk(Chunk* chunk);
void writeChunk(Chunk* chunk, uint8_t byte);
int addConstant(Chunk* chunk, Value value);

#endif
