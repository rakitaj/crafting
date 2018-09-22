using Lox;
using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using Xunit;

namespace LoxTests
{
    public class LexerIntegrationTests
    {
        public static string SourceFromFile(string filename)
        {
            var source = File.ReadAllText($".\\SourceCode\\{filename}");
            return source;
        }

        [Fact]
        public void Test_Scanner_with_basic_3_line_source()
        {
            var source = SourceFromFile("basic01.lox");
            var scanner = new Scanner(source);
            var tokens = scanner.ScanTokens();

            Assert.False(Program.HadError);
        }
        
    }
}
