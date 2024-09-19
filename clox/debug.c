#include <stdio.h>

#include "debug.h"
#include "value.h"

void disassembleChunk(Chunk *chunk, const char *name) {
    printf("== %s ==\n", name);

    for (int offset = 0; offset <chunk->count;) {
        offset = disassembleInstruction(chunk, offset); // this call returns offset of NEXT instruction (since instructions can be of diff. sizes)
    }
}

static int constantInstruction(const char* name, Chunk* chunk, int offset) {
    uint8_t constant = chunk->code[offset + 1];
    printf("%-16s %4d '", name, constant);
    printValue(chunk->constants.values[constant]);
    printf("'\n");

    return offset + 2;
}

static int simpleInstruction(const char* name, int offset) {
    printf("%s\n", name);
    return offset + 1;
}

int disassembleInstruction(Chunk *chunk, int offset) {
    printf("%04d ", offset); // where in chunk this instruction is
    // note: & = start of format specifier :: 0 = num padded with zeroes (not spaces) if shorter than specified width :: 4 = min width of output field :: d = expect integer, format as decimal num'
 if (offset > 0 && chunk->lines[offset] == chunk->lines[offset - 1]) {
     printf("  | ");
 } else {
     printf("%4d ", chunk->lines[offset]);
 }
    uint8_t instruction = chunk->code[offset]; // read single byte (opcode)
    switch (instruction) {
        case OP_RETURN:
            return simpleInstruction("OP_RETURN", offset);
        case OP_CONSTANT:
            return constantInstruction("OP_CONSTANT", chunk, offset);
        default:
            printf("Unknown opcode %d\n", instruction);
            return offset + 1;
    }
}
