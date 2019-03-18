using System;
using System.Collections.Generic;
using System.IO;
using System.Text;

namespace Lox.Tool
{
    public class GenerateAst
    {
        public static void Run(string outputDir)
        {
            if (outputDir == null)
            {
                Console.WriteLine("Usage: generate_ast <output directory>");
                Environment.Exit(1);
            }
            DefineAst(outputDir, "Expr", new List<string> {
              "Binary   : Expr left, Token operator, Expr right",
              "Grouping : Expr expression",
              "Literal  : Object value",
              "Unary    : Token operator, Expr right"
            });
        }

        private static void DefineAst(String outputDir, String baseName, List<String> types)
        {
            String path = outputDir + "/" + baseName + ".cs";
            using (var stream = new FileStream(path, FileMode.CreateNew))
            using (var writer = new StreamWriter(stream, Encoding.UTF8))
            {
                writer.WriteLine("using System;");
                writer.WriteLine("using System.Collections.Generic;");
                writer.WriteLine("namespace Lox");
                writer.WriteLine("{");
                writer.WriteLine("public abstract class " + baseName + " {");

                foreach (String type in types)
                {
                    String className = type.Split(":")[0].Trim();
                    String fields = type.Split(":")[1].Trim();
                    DefineType(writer, baseName, className, fields);
                }

                writer.WriteLine("}");
                writer.WriteLine("}");
                writer.Close();
            }
        }

        private static void DefineType(StreamWriter writer, String baseName, String className, String fieldList)
        {
            writer.WriteLine("  public static class " + className + " : " + baseName + " {");

            // Constructor.                                              
            writer.WriteLine("    " + className + "(" + fieldList + ") {");

            // Store parameters in fields.                               
            String[] fields = fieldList.Split(", ");
            foreach (String field in fields)
            {
                String name = field.Split(" ")[1];
                writer.WriteLine("      this." + name + " = " + name + ";");
            }

            writer.WriteLine("    }");

            // Fields.                                                   
            writer.WriteLine();
            foreach (String field in fields)
            {
                writer.WriteLine("    readonly " + field + ";");
            }

            writer.WriteLine("  }");
        }
    }
}
