using System;
using System.Collections.Generic;
using System.IO;

namespace Lox
{
    public class Program
    {
        public static bool HadError = false;

        public static void Main(string[] args)
        {
            if (args.Length > 1)
            {
                Console.WriteLine("Usage CSharpLox [script]");
                Environment.Exit(64);
            }
            else if (args.Length == 1)
            {
                RunFile(args[0]);
            }
            else
            {
                RunPrompt();
            }
        }

        public static void RunFile(string path)
        {
            byte[] bytes = File.ReadAllBytes(Path.GetFullPath(path));
            var source = System.Text.Encoding.Default.GetString(bytes);

            // Need to strip non visible whitespace character 65279 from the string[0] position. 
            source = source.Trim(new char[] { '\uFEFF', '\u200B' });

            Run(source);

            if (HadError)
            {
                Environment.Exit(65);
            }
        }

        public static void Run(string source)
        {
            Scanner scanner = new Scanner(source);
            List<Token> tokens = scanner.ScanTokens();

            // For now, just print the tokens.        
            foreach (Token token in tokens)
            {
                Console.WriteLine(token);
            }
        }

        public static void RunPrompt()
        {
            while (true)
            {
                Console.Write("> ");
                Run(Console.ReadLine());
                HadError = false;
            }
        }
        
        public static void Error(int line, String message)
        {
            Report(line, "", message);
        }

        private static void Report(int line, String where, String message)
        {
            Console.Error.WriteLine($"[line {line}] Error {where}: {message}");
            HadError = true;
        }
    }
}
