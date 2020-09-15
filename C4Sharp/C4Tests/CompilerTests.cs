using ConsoleApp;
using FluentAssertions;
using System;
using System.Collections.Generic;
using Xunit;

namespace C4Tests
{
    public class CompilerTests
    {
        public string LoadSourceCode(string filename)
        {
            return System.IO.File.ReadAllText($".\\Programs\\{filename}");
        }

        [Fact]
        public void Test_Lex_On_Small_Program()
        {
            var source = this.LoadSourceCode("return_2.c");
            var compiler = new Compiler(source);
            var tokens = compiler.Lex();
            var expected = new List<Token>()
            {
                new KeywordInt(), new Identifier("main"), new ParenOpen(), new ParenClose(),
                new BraceOpen(), new KeywordReturn(), new LiteralInteger("2"), new Semicolon(), new BraceClose()
            };
            tokens.Should().BeEquivalentTo(expected);
        }
    }
}
