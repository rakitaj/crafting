using System;
using System.IO;

namespace ConsoleApp
{
    public class Program
    {
        static void Main(string[] args)
        {
            var rawSource = File.ReadAllText(args[0]);
            var sourceCode = new Compiler(rawSource);
        }
    }
}
