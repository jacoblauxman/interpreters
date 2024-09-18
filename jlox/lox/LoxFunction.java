package jlox.lox;

import java.util.List;

class LoxFunction implements LoxCallable {

    private final Stmt.Function declaration;
    private final Environment closure;

    LoxFunction(Stmt.Function declaration, Environment closure) {
        this.declaration = declaration;
        this.closure = closure;
    }

    @Override
    public int arity() {
        return declaration.params.size();
    }

    @Override
    public String toString() {
        return "<fn " + declaration.name.lexeme + ">";
    }

    @Override
    public Object call(Interpreter interpreter, List<Object> arguments) {
        // Environment environment = new Enviornment(interpreter.globals); // lexical scoping vs. global
        Environment environment = new Environment(closure);
        // params core to fns, they're ENCAPSULATED by fn, no outside code (of fn) can see them - each fn gets its own env to store these vars (params)
        // each CALL specifically (without recursion breaks, for ex)
        for (int i = 0; i < declaration.params.size(); i++) {
            environment.define(
                declaration.params.get(i).lexeme,
                arguments.get(i)
            );
        }

        // interpreter.executeBlock(declaration.body, environment);
        try {
            interpreter.executeBlock(declaration.body, environment);
        } catch (Return returnValue) {
            return returnValue.value;
        }
        return null;
    }
}
