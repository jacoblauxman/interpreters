# CH 5:
```
expression -> literal
            | unary
            | binary
            | grouping ;

literal    -> NUMBER | STRING | "true" | "false" | "nil" ;
grouping   -> "(" expression ")" ;
unary      -> ( "-" | "!" ) expression ;
binary     -> expression operator expression ;
operator   -> "==" | "!=" | "<" | ">" | "<=" | ">="
            |  "+" |  "-" | "*" | "/" ;
```

# CH 6:
```
expression -> equality
equality   -> comparison ( ( "!=" | "==" ) comparison)* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term       -> factor ( ( "-" | "+" ) factor)* ;
factor     -> unary ( ( "/" | "*" ) unary)* ;
unary      -> ( "-" | "!" ) unary
            | primiary ;
primary    -> NUMBER | STRING | "true" | "false" | "nil"
            | "(" expression ")" ;
```

# CH 8:
```
program     -> statement* EOF ;
statement   -> exprStmt
             | printStmt ;
exprStmt    -> expression ";" ;
printStmt   -> "print" expression ";" ;
```

to

```
program     -> declaration* EOF ;
declaration -> varDecl
             | statement ;
statement   -> exprStmt
             | printStmt
             | block ;
exprStmt    -> expression ";" ;
printStmt   -> "print" expression ";" ;
block       -> "{" declaration* "}"
varDecl     -> "var" IDENTIFIER ( "=" expression )? ";" ;

expression -> assignment ;
assignment -> IDENTIFIER "=" assignment
            | equality ;
equality   -> comparison ( ( "!=" | "==" ) comparison)* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term       -> factor ( ( "-" | "+" ) factor)* ;
factor     -> unary ( ( "/" | "*" ) unary)* ;
unary      -> ( "-" | "!" ) unary
            | primiary ;
primary    -> NUMBER | STRING
            | "true" | "false" | "nil"
            | "(" expression ")"
            | IDENTIFIER ;
```

# CH 9:
```
program     -> declaration* EOF ;
declaration -> varDecl
             | statement ;
statement   -> exprStmt
             | forStmt
             | ifStmt
             | printStmt
             | whileStmt
             | block ;
exprStmt    -> expression ";" ;
printStmt   -> "print" expression ";" ;
block       -> "{" declaration* "}"
varDecl     -> "var" IDENTIFIER ( "=" expression )? ";" ;
ifStmt      -> "if" "(" expression ")" statement
               ( "else" statement )? ;
whileStmt   -> "while" "(" expression ")" statement ;
forStmt     -> "for" "(" ( varDecl | exprStmt | ";" )
               expression? ";"
               expression? ")" statement ;

expression -> assignment ;
assignment -> IDENTIFIER "=" assignment
            | logic_or ;
logic_or   -> logic_and ( "or" logic_and )* ;
logic_and  -> equality ( "and" equality )* ;
equality   -> comparison ( ( "!=" | "==" ) comparison)* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term       -> factor ( ( "-" | "+" ) factor)* ;
factor     -> unary ( ( "/" | "*" ) unary)* ;
unary      -> ( "-" | "!" ) unary
            | primiary ;
primary    -> NUMBER | STRING
            | "true" | "false" | "nil"
            | "(" expression ")"
            | IDENTIFIER ;
```

# CH 10:
```
program     -> declaration* EOF ;
declaration -> funDecl
             | varDecl
             | statement ;
statement   -> exprStmt
             | forStmt
             | ifStmt
             | printStmt
             | returnStmt
             | whileStmt
             | block ;
exprStmt    -> expression ";" ;
printStmt   -> "print" expression ";" ;
returnStmt  -> "return" expression? ";" ;
block       -> "{" declaration* "}"
funDecl     -> "fun" function ;
function    -> IDENTIFIER "(" parameters? ")" block ;
parameters  -> IDENTIFIER ( "," IDENTIFIER )* ;
varDecl     -> "var" IDENTIFIER ( "=" expression )? ";" ;
ifStmt      -> "if" "(" expression ")" statement
               ( "else" statement )? ;
whileStmt   -> "while" "(" expression ")" statement ;
forStmt     -> "for" "(" ( varDecl | exprStmt | ";" )
               expression? ";"
               expression? ")" statement ;

arguments  -> expression ( "," expression )* ;
expression -> assignment ;
assignment -> IDENTIFIER "=" assignment
            | logic_or ;
logic_or   -> logic_and ( "or" logic_and )* ;
logic_and  -> equality ( "and" equality )* ;
equality   -> comparison ( ( "!=" | "==" ) comparison)* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term       -> factor ( ( "-" | "+" ) factor)* ;
factor     -> unary ( ( "/" | "*" ) unary)* ;
unary      -> ( "-" | "!" ) unary
            | call ;
call       -> primary ( "(" arguments? ")" )* ;
primary    -> NUMBER | STRING
            | "true" | "false" | "nil"
            | "(" expression ")"
            | IDENTIFIER ;
```
