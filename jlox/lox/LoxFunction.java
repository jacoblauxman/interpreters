package lox;

import java.util.List;

class LoxFunction implements LoxCallable {

    private final Stmt.Function declaration;

    LoxFunction(Stmt.Function declaration) {
        this.declaration = declaration;
    }

    @Override
    public int arity() {
        return declaration.params.size();
    }

    @override
    public String toString() {
        return "<fn " + declaration.name.lexeme + ">";
    }

    @Override
    public Object call(Interpreter interpreter, List<Object> arguments) {
        Environment environment = new Enviornment(interpreter.globals);
        // params core to fns, they're ENCAPSULATED by fn, no outside code (of fn) can see them - each fn gets its own env to store these vars (params)
        // each CALL specifically (without recursion breaks, for ex)
        for (int i = 0; i < declaration.params.size(); i++) {
            environment.define(
                declaration.params.get(i).lexeme,
                arguments.get(i)
            );
        }

        interpreter.executeBlock(declaration.body, environment);
        return null;
    }
}
