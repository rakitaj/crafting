using Lox;
using System;
using System.Collections.Generic;
using Xunit;

namespace LoxTests
{
    public class LexerTests
    {
        [Fact]
        public void Test_Scanner_with_single_character()
        {
            var scanner = new Scanner("!");
            var tokens = scanner.ScanTokens();
            var expectedTokens = new List<Token> {
                new Token(TokenType.BANG, "!", null, 1),
                new Token(TokenType.EOF, "", null, 1)
            };

            Assert.Equal(expectedTokens, tokens);
        }

    }
}
