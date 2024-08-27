package lox;

// wraps return value as RTE with JVM machinery disabled (some, ex. stack traces) - unwinds all the way to where function call began (`call` in LoxFunction)
class Return extends RuntimeException {

    final Object value;

    Return(Object value) {
        super(null, null, false, false);
        this.value = value;
    }
}
