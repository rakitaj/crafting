using Lox;
using System;
using System.Collections.Generic;
using Xunit;

namespace LoxTests
{
    public class LexerTests
    {
        public Token EndOfFile(int line)
        {
            return new Token(TokenType.EOF, "", null, line);
        }

        [Fact]
        public void Test_Scanner_with_single_character()
        {
            var scanner = new Scanner("!");
            var tokens = scanner.ScanTokens();
            var expectedTokens = new List<Token> {
                new Token(TokenType.BANG, "!", null, 1),
                this.EndOfFile(1)
            };

            Assert.Equal(expectedTokens, tokens);
        }

        [Fact]
        public void Test_Scanner_with_source_code_file()
        {

        }

    }
}
