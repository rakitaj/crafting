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
        public void Test_Scanner_with_two_single_char_tokens()
        {
            var scanner = new Scanner("!+");
            var tokens = scanner.ScanTokens();
            var expectedTokens = new List<Token>
            {
                new Token(TokenType.BANG, "!", null, 1),
                new Token(TokenType.PLUS, "+", null, 1),
                this.EndOfFile(1)
            };

            Assert.Equal(expectedTokens, tokens);
        }

        [Fact]
        public void Test_Scanner_with_multiple_single_char_tokens()
        {
            var scanner = new Scanner("+-*<>");
            var tokens = scanner.ScanTokens();
            var expectedTokens = new List<Token>
            {
                new Token(TokenType.PLUS, "+", null, 1),
                new Token(TokenType.MINUS, "-", null, 1),
                new Token(TokenType.STAR, "*", null, 1),
                new Token(TokenType.LESS, "<", null, 1),
                new Token(TokenType.GREATER, ">", null, 1),
                this.EndOfFile(1)
            };

            Assert.Equal(expectedTokens, tokens);
        }

    }
}
