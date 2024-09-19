package jlox.lox;

import java.util.HashMap;
import java.util.Map;

class LoxInstance {

    private LoxClass klass;
    private final Map<String, Object> fields = new HashMap<>();

    LoxInstance(LoxClass klass) {
        this.klass = klass;
    }

    // no nil/undefined return if !containsKey
    Object get(Token name) {
        if (fields.containsKey(name.lexeme)) {
            return fields.get(name.lexeme);
        }

        throw new RuntimeError(
            name,
            "Undefined propert '" + name.lexeme + "'."
        );
    }

    @Override
    public String toString() {
        return klass.name + " instance";
    }
}
// Note 'property' vs 'field':
// field = named bits of state stored directly in instance
// property = named 'things' that get expr may return
// - every field is a property, not every property is a field
