package lox;

import java.util.HashMap;
import java.util.Map;

class Environment {

    final Environment enclosing;
    private final Map<String, Object> values = new HashMap<>();

    // global scope
    Environment() {
        enclosing = null;
    }

    // block scope
    Environment(Environment enclosing) {
        this.enclosing = enclosing;
    }

    void define(String name, Object value) {
        values.put(name, value);
    }

    Object get(Token name) {
        if (values.containsKey(name.lexeme)) {
            return values.get(name.lexeme);
        }

        if (enclosing != null) return enclosing.get(name); // walk to outer scope

        throw new RuntimeError(
            name,
            "Undefined variable '" + name.lexeme + "'."
        );
    }

    void assign(Token name, Object value) {
        if (values.containsKey(name.lexeme)) {
            values.put(name.lexeme, value);
            return;
        }

        if (enclosing != null) {
            enclosing.assign(name, value); // walk to outer scope
            return;
        }

        throw new RuntimeError(
            name,
            "Undefined variable '" + name.lexeme + "'."
        );
    }
}
