package jlox.lox;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Stack;

class Resolver implements Expr.Visitor<Void>, Stmt.Visitor<Void> {

    private final Interpreter interpreter;
    private final Stack<Map<String, Boolean>> scopes = new Stack<>();
    private FunctionType currentFunction = FunctionType.NONE;

    private enum FunctionType {
        NONE,
        FUNCTION,
    }

    // handles:
    // - variable decl, read, written
    // - scope creation and destruction
    Resolver(Interpreter interpreter) {
        this.interpreter = interpreter;
    }

    void resolve(List<Stmt> statements) {
        for (Stmt statement : statements) {
            resolve(statement);
        }
    }

    private void beginScope() {
        scopes.push(new HashMap<String, Boolean>());
    }

    private void endScope() {
        scopes.pop();
    }

    private void declare(Token name) {
        if (scopes.isEmpty()) return;

        Map<String, Boolean> scope = scopes.peek();
        if (scope.containsKey(name.lexeme)) {
            Lox.error(
                name,
                "Already a variable with this name in the current scope."
            );
        }
        scope.put(name.lexeme, false);
    }

    // resolve initializer expr in same scope where new var exists but unavailable
    private void define(Token name) {
        if (scopes.isEmpty()) return;
        scopes.peek().put(name.lexeme, true); // var fully init'd within given scope map
    }

    // similar to Environment re: evaluating var:
    // - work outwards from innermost scope
    // - if var found resolve, along w/ scope idx where found (curr = 0, prev enclosing = 1, etc.)
    // - else, assumed global (+ left unresolved)
    private void resolveLocal(Expr expr, Token name) {
        for (int i = scopes.size() - 1; i >= 0; i--) {
            if (scopes.get(i).containsKey(name.lexeme)) {
                interpreter.resolve(expr, scopes.size() - 1 - i);
                return;
            }
        }
    }

    @Override
    public Void visitBlockStmt(Stmt.Block stmt) {
        beginScope();
        resolve(stmt.statements);
        endScope();
        return null;
    }

    @Override
    public Void visitClassStmt(Stmt.Class stmt) {
        declare(stmt.name);
        define(stmt.name);
        return null;
    }

    @Override
    public Void visitExpressionStmt(Stmt.Expression stmt) {
        resolve(stmt.expression);
        return null;
    }

    @Override
    public Void visitIfStmt(Stmt.If stmt) {
        resolve(stmt.condition);
        resolve(stmt.thenBranch);
        if (stmt.elseBranch != null) resolve(stmt.elseBranch);
        return null;
    }

    @Override
    public Void visitPrintStmt(Stmt.Print stmt) {
        resolve(stmt.expression);
        return null;
    }

    @Override
    public Void visitReturnStmt(Stmt.Return stmt) {
        // confirm within fn decl
        if (currentFunction == FunctionType.NONE) {
            Lox.error(stmt.keyword, "Can't return from top-level code.");
        }
        if (stmt.value != null) {
            resolve(stmt.value);
        }
        return null;
    }

    @Override
    public Void visitVarStmt(Stmt.Var stmt) {
        // adds var to innermost scope, shadows any outer one and know it exists as a var
        // - marked as 'not ready' w/ falsey binding (scope map)
        // - the key's val represents whether or not we've finished resolving the var's initializer
        declare(stmt.name);
        if (stmt.initializer != null) {
            // init expr in same scope where var exists but 'unavailable'
            resolve(stmt.initializer);
        }
        // now available within scope map
        define(stmt.name);
        return null;
    }

    @Override
    public Void visitWhileStmt(Stmt.While stmt) {
        resolve(stmt.condition);
        resolve(stmt.body);
        return null;
    }

    @Override
    public Void visitVariableExpr(Expr.Variable expr) {
        if (
            !scopes.isEmpty() &&
            scopes.peek().get(expr.name.lexeme) == Boolean.FALSE
        ) {
            Lox.error(
                expr.name,
                "Cant read local variabile in its own initializer."
            );
        }

        resolveLocal(expr, expr.name);
        return null;
    }

    @Override
    public Void visitAssignExpr(Expr.Assign expr) {
        resolve(expr.value); // resolve expr for assigned val in case ref's to other vars
        resolveLocal(expr, expr.name); // resolve var being assigned to
        return null;
    }

    @Override
    public Void visitBinaryExpr(Expr.Binary expr) {
        resolve(expr.left);
        resolve(expr.right);
        return null;
    }

    @Override
    public Void visitCallExpr(Expr.Call expr) {
        resolve(expr.callee);

        for (Expr argument : expr.arguments) {
            resolve(argument);
        }

        return null;
    }

    @Override
    public Void visitGetExpr(Expr.Get expr) {
        // properties looked up dynamically, don't get resoled - recurse only into expression to left of 'dot' - property access happens w/ interpreterk
        resolve(expr.object);
        return null;
    }

    @Override
    public Void visitGroupingExpr(Expr.Grouping expr) {
        resolve(expr.expression);
        return null;
    }

    @Override
    public Void visitLiteralExpr(Expr.Literal expr) {
        return null; // doesn't mention any vars and doesn't contain sub-expr == nothing to do
    }

    @Override
    public Void visitLogicalExpr(Expr.Logical expr) {
        resolve(expr.left);
        resolve(expr.right);
        return null;
    }

    @Override
    public Void visitUnaryExpr(Expr.Unary expr) {
        resolve(expr.right);
        return null;
    }

    @Override
    public Void visitFunctionStmt(Stmt.Function stmt) {
        declare(stmt.name);
        define(stmt.name);
        // decl + def like vistVarStmt, BUT define name eagerly, before resolving fn body
        // allows fn to ro recursively refer to itself within its body
        resolveFunction(stmt, FunctionType.FUNCTION);
        return null;
    }

    // both bind names and introduce scope

    private void resolve(Stmt stmt) {
        stmt.accept(this);
    }

    private void resolve(Expr expr) {
        expr.accept(this);
    }

    // separate (later used for methods w/ classes)
    // resolves fn body in the scope created - different from interpreter handling fn decl
    // - RUNTIME fn decl doesn't 'run' body until fn is called
    // - STATIC ANALYSIS ('this') immediately traverses body
    private void resolveFunction(Stmt.Function function, FunctionType type) {
        // stash prev value of field in local var first (for local fns, can nest fn decl)
        // need to track 'how many fns in'
        FunctionType enclosingFunction = currentFunction; // save state of 'this'
        currentFunction = type;

        beginScope();
        for (Token param : function.params) {
            declare(param);
            define(param);
        }
        resolve(function.body);
        endScope();
        currentFunction = enclosingFunction; // after resolving fn body, restore field
    }
}
