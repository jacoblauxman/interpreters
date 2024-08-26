package lox;

import static lox.TokenType.*;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

class Scanner {

    private final String source;
    private final List<Token> tokens = new ArrayList<>();
    private int start = 0; // points to first char in lexeme being scanned
    private int current = 0; // points at curr char being considered
    private int line = 1; // helps produce tokens that know their location

    // for handling keywords re: identifier's lexeme:
    private static final Map<String, TokenType> keywords;

    static {
        keywords = new HashMap<>();
        keywords.put("and", AND);
        keywords.put("class", CLASS);
        keywords.put("else", ELSE);
        keywords.put("false", FALSE);
        keywords.put("for", FOR);
        keywords.put("fun", FUN);
        keywords.put("if", IF);
        keywords.put("nil", NIL);
        keywords.put("or", OR);
        keywords.put("print", PRINT);
        keywords.put("return", RETURN);
        keywords.put("super", SUPER);
        keywords.put("this", THIS);
        keywords.put("true", TRUE);
        keywords.put("var", VAR);
        keywords.put("while", WHILE);
    }

    Scanner(String source) {
        this.source = source;
    }

    List<Token> scanTokens() {
        while (!isAtEnd()) {
            // beginning of next lexeme
            start = current;
            scanToken();
        }

        tokens.add(new Token(EOF, "", null, line));
        return tokens;
    }

    private void scanToken() {
        char c = advance();

        switch (c) {
            case '(':
                addToken(LEFT_PAREN);
                break;
            case ')':
                addToken(RIGHT_PAREN);
                break;
            case '{':
                addToken(LEFT_BRACE);
                break;
            case '}':
                addToken(RIGHT_BRACE);
                break;
            case ',':
                addToken(COMMA);
                break;
            case '.':
                addToken(DOT);
                break;
            case '-':
                addToken(MINUS);
                break;
            case '+':
                addToken(PLUS);
                break;
            case ';':
                addToken(SEMICOLON);
                break;
            case '*':
                addToken(STAR);
                break;
            // THESE cases need to look at the second char (two char operators)
            case '!':
                addToken(match('=') ? BANG_EQUAL : BANG);
                break;
            case '=':
                addToken(match('=') ? EQUAL_EQUAL : EQUAL);
                break;
            case '<':
                addToken(match('=') ? LESS_EQUAL : LESS);
                break;
            case '>':
                addToken(match('=') ? GREATER_EQUAL : GREATER);
                break;
            // handling of both comments vs. division op
            case '/':
                if (match('/')) {
                    // comment goes until the end of line
                    while (peek() != '\n' && !isAtEnd()) advance();
                    // note: we use the LOOKAHEAD of `peek` to determine end of comment - we do NOT call addToken (the comment is not 'digested' further)
                } else {
                    addToken(SLASH);
                }
                break;
            // 'meaningless chars' (whitespace/newlines) -- reminder without break we can 'fall through' all cases and THEN break
            case ' ':
            case '\r':
            case '\t':
                break;
            // except newlines, we want to increment `line` val
            case '\n':
                line++;
                break;
            case '"':
                string();
                break;
            // for all float point/number instances and unknown chars error handling
            // re: errors-- at lexical level, what if source file has chars Lox doesn't use?
            default:
                if (isDigit(c)) {
                    number();
                } else if (isAlpha(c)) {
                    identifier();
                } else {
                    Lox.error(line, "Unexpected char.");
                }
                break;
            // NOTE: erroneous char is still consumed by call in advance
            // - avoids infinite loop ^
            // - we keep scanning, in case other errors we can report all found in 'one go' (no syntax WACK A MOLE)
            // sets HadError -- no code gets EXECUTED
        }
    }

    private void identifier() {
        while (isAlphaNumeric(peek())) advance();

        String text = source.substring(start, current);

        TokenType type = keywords.get(text);
        if (type == null) type = IDENTIFIER;

        addToken(type);
    }

    private void number() {
        while (isDigit(peek())) advance();

        // look for decimal
        if (peek() == '.' && isDigit(peekNext())) {
            // consume '.'
            advance();

            // find all post decimal numeric vals ('fractional' part)
            while (isDigit(peek())) advance();
        }

        addToken(NUMBER, Double.parseDouble(source.substring(start, current)));
    }

    private void string() {
        while (peek() != '"' && !isAtEnd()) {
            if (peek() == '\n') line++;
            advance();
        }

        if (isAtEnd()) {
            Lox.error(line, "Unterminated string.");
            return;
        }

        // to closing '"'
        advance();

        // remove quotes, return the token with the additional literal value (needed later steps)
        String value = source.substring(start + 1, current - 1);
        addToken(STRING, value);
    }

    // a 'conditional' version of `advance`
    private boolean match(char expected) {
        if (isAtEnd()) return false;
        if (source.charAt(current) != expected) return false;

        current++;
        return true;
    }

    // for LOOKAHEAD (without consuming char)
    private char peek() {
        if (isAtEnd()) return '\0';
        return source.charAt(current);
    }

    // for looking past decimal point specifically (need '2' places ahead)
    private char peekNext() {
        if (current + 1 >= source.length()) return '\0';
        return source.charAt(current + 1);
    }

    private boolean isAlpha(char c) {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    private boolean isAlphaNumeric(char c) {
        return isAlpha(c) || isDigit(c);
    }

    private boolean isDigit(char c) {
        return c >= '0' && c <= '9';
    }

    // checks if all chars consumed
    private boolean isAtEnd() {
        return current >= source.length();
    }

    // consumes next char from source and returns (for input)
    private char advance() {
        return source.charAt(current++);
    }

    // grabs text at curr lexeme and creates new token (for output)
    private void addToken(TokenType type) {
        addToken(type, null);
    }

    // grabs text at curr lexeme and allows for overload to handle tokens with LITERAL vals
    private void addToken(TokenType type, Object literal) {
        String text = source.substring(start, current);
        tokens.add(new Token(type, text, literal, line));
    }
}
