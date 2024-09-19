#include "chunk.h"
#include "common.h"
#include "debug.h"
#include "value.h"
#include "vm.h"
#include <stdio.h>

VM vm;

static void resetStack() {
    vm.stackTop = vm.stack; // stack arr is declared inline for VM, no need to allocate (set to point to beg of said arr)
}

void initVM() {
resetStack();
}

void freeVM() {

}

void push(Value value) {
    *vm.stackTop = value; // store at top of stack (just past last used element, next available one)
    vm.stackTop++; // ptr to next unused slot in arr
}

Value pop() {
    vm.stackTop--; // move ptr to more recently used
    return *vm.stackTop; // item, stackTop will overwrite next `push` (unless stack overflow)
}

static InterpretResult run() {
    #define READ_BYTE() (*vm.ip++) // macro reads byte currently pointed at by ip, then advances ip (first byte of any instruction is op code)
    // note: the '++' AFTER deref of vm.ip value moves it forward after switch case

    #define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()]) // treats res of READ_BYTE as idx of `Value` to look up from chunk

    // each loop executes a single byte instruction
    for (;;) {
        #ifdef DEBUG_TRACE_EXECUTION
            printf("        ");
            // loop to print each value ins tack arr from bot to top (observe effects of instructions)
            for (Value* slot = vm.stack; slot < vm.stackTop; slot++) {
                printf("[ ");
                printValue(*slot);
                printf(" ");
            }
            printf("\n");
            disassembleInstruction(vm.chunk, (int)(vm.ip - vm.chunk->code)); // need integer offset, vm.ip is current instr. ref (direct ptr), convert it back relative to beginning of bytecode (curr - src)
        #endif

        uint8_t instruction;
        switch (instruction = READ_BYTE()) {
            case OP_CONSTANT: {
                Value constant = READ_CONSTANT();
                printValue(constant); // for now just print constant's value
                printf("\n");
                break;
            }
            case OP_RETURN: {
                printValue(pop()); // temp. means for executing simple instr. sequence + display
                printf("\n");
                return INTERPRET_OK; // no function yet, so for now just end execution
            }
        }
    }

    #undef READ_BYTE
    #undef READ_CONSTANT
}

InterpretResult interpret(Chunk* chunk) {
    vm.chunk = chunk;
    vm.ip = vm.chunk->code;
    return run();
}
