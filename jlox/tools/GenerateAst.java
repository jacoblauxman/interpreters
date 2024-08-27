package tools;

import java.io.IOException;
import java.io.PrintWriter;
import java.util.Arrays;
import java.util.List;

public class GenerateAst {

    public static void main(String[] args) throws IOException {
        if (args.length != 1) {
            System.err.println("Usage: generate_ast <output directory>");
            System.exit(64);
        }
        String outputDir = args[0];

        // generates classes for expressions (rules/productions)
        defineAst(
            outputDir,
            "Expr",
            Arrays.asList(
                // ch 8 - statements
                "Assign : Token name, Expr value",
                //
                "Binary   : Expr left, Token operator, Expr right",
                // Ch 10
                "Call     : Expr callee, Token paren, List<Expr> arguments",
                //
                "Grouping : Expr expression",
                "Literal  : Object value",
                // Ch 9 - control flow
                "Logical  : Expr left, Token operator, Expr right",
                //
                "Unary    : Token operator, Expr right",
                // Ch 8 - statements
                "Variable : Token name"
            )
        );
        // added with ch 8 - our statements (expression statements + print statements)
        defineAst(
            outputDir,
            "Stmt",
            Arrays.asList(
                "Block : List<Stmt> statements",
                "Expression : Expr expression",
                "Print : Expr expression",
                "Var : Token name, Expr initializer",
                // Ch 9 - control flow
                "If  : Expr condition, Stmt thenBranch," + " Stmt elseBranch",
                "While : Expr condition, Stmt body",
                //
                "Function : Token name, List<Token> params," +
                " List<Stmt> body"
            )
        );
    }

    //
    private static void defineAst(
        String outputDir,
        String baseName,
        List<String> types
    ) throws IOException {
        String path = outputDir + "/" + baseName + ".java";
        PrintWriter writer = new PrintWriter(path, "UTF-8");

        // writer.println("package craftinginterpreters.lox;");
        writer.println("package lox;");
        writer.println();
        writer.println("import java.util.List;");
        writer.println();
        writer.println("abstract class " + baseName + " {");

        // define the visitor interface
        defineVisitor(writer, baseName, types);

        // The AST classes.
        for (String type : types) {
            String className = type.split(":")[0].trim();
            String fields = type.split(":")[1].trim();
            defineType(writer, baseName, className, fields);
        }

        // the base `accept` method
        writer.println();
        writer.println("  abstract <R> R accept(Visitor<R> visitor);");

        writer.println("}");
        writer.close();
    }

    // generates visitor interface
    private static void defineVisitor(
        PrintWriter writer,
        String baseName,
        List<String> types
    ) {
        writer.println("  interface Visitor<R> {");

        for (String type : types) {
            String typeName = type.split(":")[0].trim();
            writer.println(
                "    R visit" +
                typeName +
                baseName +
                "(" +
                typeName +
                " " +
                baseName.toLowerCase() +
                ");"
            );
        }

        writer.println("  }");
    }

    private static void defineType(
        PrintWriter writer,
        String baseName,
        String className,
        String fieldList
    ) {
        writer.println(
            "  static class " + className + " extends " + baseName + " {"
        );

        // Constructor.
        writer.println("    " + className + "(" + fieldList + ") {");

        // Store parameters in fields.
        String[] fields = fieldList.split(", ");
        for (String field : fields) {
            String name = field.split(" ")[1];
            writer.println("      this." + name + " = " + name + ";");
        }

        writer.println("    }");

        // Visitor pattern.
        writer.println();
        writer.println("    @Override");
        writer.println("    <R> R accept(Visitor<R> visitor) {");
        writer.println(
            "      return visitor.visit" + className + baseName + "(this);"
        );
        writer.println("    }");

        // Fields.
        writer.println();
        for (String field : fields) {
            writer.println("    final " + field + ";");
        }

        writer.println("  }");
    }
}
