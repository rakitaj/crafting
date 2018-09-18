using System;
using System.IO;

namespace Lox
{
    class Program
    {
        static void Main(string[] args)
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
            Run(source);
        }

        public static void Run(string source)
        {

        }

        public static void RunPrompt()
        {
            while (true)
            {
                Console.Write("> ");
                Run(Console.ReadLine());
            }
        }
    }
}
