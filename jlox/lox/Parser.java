package lox;

import static lox.TokenType.*;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

class Parser {

    private static class ParseError extends RuntimeException {}

    private final List<Token> tokens;
    private int current = 0;

    Parser(List<Token> tokens) {
        this.tokens = tokens;
    }

    // Ch 6-7: Parsing + Evaluating Expressions
    // Expr parse() {
    //     try {
    //         return expression();
    //     } catch (ParseError error) {
    //         return null;
    //     }
    // }

    // Ch 8: Statements update:
    List<Stmt> parse() {
        List<Stmt> statements = new ArrayList<>();
        while (!isAtEnd()) {
            // statements.add(statement());
            statements.add(declaration());
        }

        return statements;
    }

    private Expr expression() {
        // return equality(); // updated w/ ch 8 statements
        return assignment();
    }

    // Ch 8
    private Stmt declaration() {
        // note: highest level 'Stmt' - sync when in 'panic mode'
        // -> try/catch leads to error recovery, then tries to parse next statement
        try {
            if (match(FUN)) return function("function");
            if (match(VAR)) return varDeclaration();

            return statement();
        } catch (ParseError error) {
            synchronize();
            return null;
        }
    }

    private Stmt varDeclaration() {
        Token name = consume(IDENTIFIER, "Expect variable name.");

        Expr initializer = null;
        if (match(EQUAL)) {
            initializer = expression();
        }

        consume(SEMICOLON, "Expect ';' after variable declaration.");

        return new Stmt.Var(name, initializer);
    }

    // Ch 9 - control flow
    private Stmt whileStatement() {
        consume(LEFT_PAREN, "Expect '(' after 'while'.");
        Expr condition = expression();
        consume(RIGHT_PAREN, "Expect ')' after condition.");
        Stmt body = statement();

        return new Stmt.While(condition, body);
    }

    // Ch 9
    private Stmt statement() {
        // Ch 9
        if (match(FOR)) return forStatement();
        if (match(IF)) return ifStatement();
        if (match(WHILE)) return whileStatement();
        //
        if (match(PRINT)) return printStatement();
        if (match(LEFT_BRACE)) return new Stmt.Block(block());
        return expressionStatement();
    }

    private Stmt forStatement() {
        consume(LEFT_PAREN, "Expect '(' after 'for'.");

        Stmt initializer;
        if (match(SEMICOLON)) {
            initializer = null;
        } else if (match(VAR)) {
            initializer = varDeclaration();
        } else {
            initializer = expressionStatement();
        }

        Expr condition = null;
        if (!check(SEMICOLON)) {
            condition = expression();
        }

        consume(SEMICOLON, "Expect ';' after loop condition.");

        Expr increment = null;
        if (!check(RIGHT_PAREN)) {
            increment = expression();
        }

        consume(RIGHT_PAREN, "Expect ')' after 'for' clauses.");

        Stmt body = statement();

        if (increment != null) {
            body = new Stmt.Block(
                Arrays.asList(body, new Stmt.Expression(increment))
            );
        }

        if (condition == null) condition = new Expr.Literal(true); // always true / inf. loop if ommitted
        body = new Stmt.While(condition, body);

        if (initializer != null) {
            body = new Stmt.Block(Arrays.asList(initializer, body));
        }

        return body;
    }

    private Stmt ifStatement() {
        consume(LEFT_PAREN, "Expect '(' after 'if'.");
        Expr condition = expression();
        consume(RIGHT_PAREN, "Expect ')' after if condition.");

        Stmt thenBranch = statement();
        Stmt elseBranch = null;

        // avoid 'dangling else': eagerly looks for `else` before return, nested 'if' stmt will claim clause (before return to outer 'if')
        if (match(ELSE)) {
            elseBranch = statement();
        }

        return new Stmt.If(condition, thenBranch, elseBranch);
    }

    private Stmt printStatement() {
        Expr value = expression();
        consume(SEMICOLON, "Expect ';' after value.");

        return new Stmt.Print(value);
    }

    private Stmt expressionStatement() {
        Expr expr = expression();
        consume(SEMICOLON, "Expect ';' after expression.");

        return new Stmt.Expression(expr);
    }

    private Stmt.Function function(String kind) {
        Token name = consume(IDENTIFIER, "Expect " + kind + " name.");
        consume(LEFT_PAREN, "Expect '(' after " + kind + " name.");
        List<Token> parameters = new ArrayList<>();
        if (!check(RIGHT_PAREN)) {
            do {
                if (parameters.size() >= 255) {
                    error(peek(), "Can't have more than 255 parameters.");
                }

                parameters.add(consume(IDENTIFIER, "Expect parameter name."));
            } while (match(COMMA));
        }
        consume(RIGHT_PAREN, "Expect ')' after parameters.");

        consume(LEFT_BRACE, "Expect '{' before " + kind + " body.");
        List<Stmt> body = block();

        return new Stmt.Function(name, parameters, body);
    }

    private List<Stmt> block() {
        List<Stmt> statements = new ArrayList<>();

        while (!check(RIGHT_BRACE) && !isAtEnd()) {
            statements.add(declaration());
        }

        consume(RIGHT_BRACE, "Expect '}' after block.");

        return statements;
    }

    private Expr assignment() {
        // Expr expr = equality(); // Ch 9
        Expr expr = or();
        //
        if (match(EQUAL)) {
            Token equals = previous();
            Expr value = assignment();

            if (expr instanceof Expr.Variable) {
                Token name = ((Expr.Variable) expr).name;

                return new Expr.Assign(name, value);
            }

            error(equals, "Invalid assignment target.");
        }

        return expr;
    }

    // Ch 8

    // Ch 9
    private Expr or() {
        Expr expr = and();

        while (match(OR)) {
            Token operator = previous();
            Expr right = and();
            expr = new Expr.Logical(expr, operator, right);
        }

        return expr;
    }

    private Expr and() {
        Expr expr = equality();

        while (match(AND)) {
            Token operator = previous();
            Expr right = equality();
            expr = new Expr.Logical(expr, operator, right);
        }

        return expr;
    }

    // Ch 9

    private Expr equality() {
        // note: effectively matches an 'equality' operator OR 'anything of higher precedence'
        Expr expr = comparison();

        while (match(BANG_EQUAL, EQUAL_EQUAL)) {
            Token operator = previous();
            Expr right = comparison();
            expr = new Expr.Binary(expr, operator, right);
        }

        return expr;
    }

    private Expr comparison() {
        Expr expr = term();

        while (match(GREATER, GREATER_EQUAL, LESS, LESS_EQUAL)) {
            Token operator = previous();
            Expr right = term();
            expr = new Expr.Binary(expr, operator, right);
        }

        return expr;
    }

    private Expr term() {
        Expr expr = factor();

        while (match(MINUS, PLUS)) {
            Token operator = previous();
            Expr right = factor();
            expr = new Expr.Binary(expr, operator, right);
        }

        return expr;
    }

    private Expr factor() {
        Expr expr = unary();

        while (match(SLASH, STAR)) {
            Token operator = previous();
            Expr right = unary();
            expr = new Expr.Binary(expr, operator, right);
        }

        return expr;
    }

    private Expr unary() {
        if (match(BANG, MINUS)) {
            Token operator = previous();
            Expr right = unary();
            return new Expr.Unary(operator, right);
        }

        // return primary(); // Ch 10
        return call();
    }

    // Ch 10
    private Expr finishCall(Expr callee) {
        List<Expr> arguments = new ArrayList<>();
        if (!check(RIGHT_PAREN)) {
            do {
                // for sake of compat. with bytecode interpreter
                if (arguments.size() >= 255) {
                    error(peek(), "Cant have more than 255 arguments."); // no throw, parser isn't 'confused/invalid state', report and continue
                }
                arguments.add(expression());
            } while (match(COMMA));
        }

        Token paren = consume(
            RIGHT_PAREN,
            "Expect ')' after callee's arguments."
        );

        return new Expr.Call(callee, paren, arguments);
    }

    private Expr call() {
        // - similar to parse infix op's -
        // parse primary "left operand" to call
        // -> each '(' use 'finishCall' to parse call expr using prev. parsed expr as callee
        // -> return expr = expr, loop -^
        Expr expr = primary();

        // note: while true / break for later (obj properties)
        while (true) {
            if (match(LEFT_PAREN)) {
                expr = finishCall(expr);
            } else {
                break;
            }
        }

        return expr;
    }

    private Expr primary() {
        if (match(TRUE)) return new Expr.Literal(true);
        if (match(FALSE)) return new Expr.Literal(false);
        if (match(NIL)) return new Expr.Literal(null);

        if (match(NUMBER, STRING)) {
            return new Expr.Literal(previous().literal);
        }
        // ch 8 - statements
        if (match(IDENTIFIER)) {
            return new Expr.Variable(previous());
        }
        //
        if (match(LEFT_PAREN)) {
            Expr expr = expression();
            consume(RIGHT_PAREN, "Expect ')' after expression.");
            return new Expr.Grouping(expr);
        }

        throw error(peek(), "Expect expression.");
    }

    // === HELPERS === //

    // checks if current token has any of the given types (like in `equality` for "!=" and "==" operators || in `comparison` for (any) comparison operators)
    // - true: 'consumes' token (advances)
    // - false: leaves current token alone (no advance)
    private boolean match(TokenType... types) {
        for (TokenType type : types) {
            if (check(type)) {
                advance();
                return true;
            }
        }

        return false;
    }

    private Token consume(TokenType type, String message) {
        if (check(type)) return advance();
        throw error(peek(), message);
    }

    private ParseError error(Token token, String message) {
        Lox.error(token, message);
        return new ParseError();
    }

    // for after catching `ParseError`:
    // discards tokens until potential statement boundary (semicolon, various keywords to init statements)
    // -- likely tokens that would have caused errors (additionally), parse rest of file beginning at next statement
    private void synchronize() {
        advance();

        while (!isAtEnd()) {
            if (previous().type == SEMICOLON) return;

            switch (peek().type) {
                case CLASS:
                case FUN:
                case VAR:
                case FOR:
                case IF:
                case WHILE:
                case PRINT:
                case RETURN:
                    return;
            }

            advance();
        }
    }

    // unlike `match` just "looks" at token to determine if current token is the given `TokenType`
    private boolean check(TokenType type) {
        if (isAtEnd()) return false;

        return peek().type == type;
    }

    // 'consumes' current token + returns (like `Scanner`'s method)
    private Token advance() {
        if (!isAtEnd()) current++;
        return previous();
    }

    // checks for end of token stream
    private boolean isAtEnd() {
        return peek().type == EOF;
    }

    // returns current token (not 'consumed')
    private Token peek() {
        return tokens.get(current);
    }

    // returns prior token (most recently 'consumed')
    private Token previous() {
        return tokens.get(current - 1);
    }
}
// NOTE: general syntax tree's expression grammar:
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;
