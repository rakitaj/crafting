using Lox;
using System;
using System.Collections.Generic;
using Xunit;

namespace LoxTests
{
    public class LexerUnitTests
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

        [Fact]
        public void Test_Scanner_with_one_double_char_token()
        {
            var scanner = new Scanner("!=");
            var tokens = scanner.ScanTokens();
            var expectedTokens = new List<Token> {
                new Token(TokenType.BANG_EQUAL, "!=", null, 1),
                this.EndOfFile(1)
            };

            Assert.Equal(expectedTokens, tokens);
        }

        [Fact]
        public void Test_scanner_many_tokens_one_line()
        {
            var scanner = new Scanner("!*+-/=<> <= == // operators");
            var tokens = scanner.ScanTokens();
            var expectedTokens = new List<Token> {
                new Token(TokenType.BANG, "!", null, 1),
                new Token(TokenType.STAR, "*", null, 1),
                new Token(TokenType.PLUS, "+", null, 1),
                new Token(TokenType.MINUS, "-", null, 1),
                new Token(TokenType.SLASH, "/", null, 1),
                new Token(TokenType.EQUAL, "=", null, 1),
                new Token(TokenType.LESS, "<", null, 1),
                new Token(TokenType.GREATER, ">", null, 1),
                new Token(TokenType.LESS_EQUAL, "<=", null, 1),
                new Token(TokenType.EQUAL_EQUAL, "==", null, 1),
                this.EndOfFile(1)
            };

            Assert.Equal(expectedTokens, tokens);
        }

        [Fact]
        public void Test_scanner_with_comment_and_whitespace()
        {
            var scanner = new Scanner("// this is a comment");
            var tokens = scanner.ScanTokens();
            var expectedTokens = new List<Token>
            {
                this.EndOfFile(1)
            };

            Assert.Equal(expectedTokens, tokens);
        }

        [Fact]
        public void Test_scanner_with_two_lines()
        {
            var scanner = new Scanner("// this is a comment\n(( )){ } // grouping stuff");
            var tokens = scanner.ScanTokens();
            var expectedTokens = new List<Token>
            {
                new Token(TokenType.LEFT_PAREN, "(", null, 2),
                new Token(TokenType.LEFT_PAREN, "(", null, 2),
                new Token(TokenType.RIGHT_PAREN, ")", null, 2),
                new Token(TokenType.RIGHT_PAREN, ")", null, 2),
                new Token(TokenType.LEFT_BRACE, "{", null, 2),
                new Token(TokenType.RIGHT_BRACE, "}", null, 2),
                this.EndOfFile(2)
            };

            Assert.Equal(expectedTokens, tokens);
            Assert.False(Program.HadError);
        }

        [Theory]
        [InlineData('0', true)]
        [InlineData('1', true)]
        [InlineData('2', true)]
        [InlineData('3', true)]
        [InlineData('4', true)]
        [InlineData('5', true)]
        [InlineData('6', true)]
        [InlineData('7', true)]
        [InlineData('8', true)]
        [InlineData('9', true)]
        [InlineData('a', false)]
        [InlineData('A', false)]
        public void Test_IsDigit(char c, bool expectedResult)
        {
            var result = Scanner.IsDigit(c);
            Assert.Equal(expectedResult, result);
        }

        [Theory]
        [InlineData('_', true)]
        [InlineData('a', true)]
        [InlineData('b', true)]
        [InlineData('c', true)]
        [InlineData('X', true)]
        [InlineData('Y', true)]
        [InlineData('Z', true)]
        [InlineData('0', false)]
        [InlineData('1', false)]
        [InlineData('5', false)]
        [InlineData('9', false)]
        [InlineData('!', false)]
        [InlineData('/', false)]
        [InlineData('.', false)]
        [InlineData('-', false)]
        public void Test_IsAlpha_Or_Underscore(char c, bool expectedResult)
        {
            var result = Scanner.IsAlpha(c);
            Assert.Equal(expectedResult, result);
        }

        [Fact]
        public void Test_scanner_with_number()
        {
            var scanner = new Scanner("123.456");
            var tokens = scanner.ScanTokens();
            var expectedTokens = new List<Token>
            {
                new Token(TokenType.NUMBER, "123.456", 123.456, 1),
                this.EndOfFile(1)
            };

            Assert.Equal(expectedTokens, tokens);
            Assert.False(Program.HadError);
        }

        [Fact]
        public void Test_Scanner_with_string()
        {
            var scanner = new Scanner("\"hello\"");
            var tokens = scanner.ScanTokens();
            var expectedTokens = new List<Token>
            {
                new Token(TokenType.STRING, "\"hello\"", "hello", 1),
                this.EndOfFile(1)
            };

            Assert.Equal(expectedTokens, tokens);
            Assert.False(Program.HadError);
        }

    }
}
