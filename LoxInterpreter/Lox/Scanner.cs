using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Lox
{
    public class Scanner
    {
        private readonly String _source;                                            
        private readonly List<Token> _tokens = new List<Token>();
        private int _start = 0;
        private int _current = 0;
        private int _line = 1;

        public Scanner(String source)
        {
            this._source = source;
        }

        private bool IsAtEnd()
        {
            return this._current >= this._source.Length;
        }

        public List<Token> ScanTokens()
        {
            while (!this.IsAtEnd())
            {
                // We are at the beginning of the next lexeme.
                this._start = this._current;
                this.ScanToken();
            }

            this._tokens.Add(new Token(TokenType.EOF, "", null, this._line));
            return this._tokens;
        }

        private void ScanToken()
        {
            char c = this.Advance();
            switch (c)
            {
                case '(': this.AddToken(TokenType.LEFT_PAREN); break;
                case ')': this.AddToken(TokenType.RIGHT_PAREN); break;
                case '{': this.AddToken(TokenType.LEFT_BRACE); break;
                case '}': this.AddToken(TokenType.RIGHT_BRACE); break;
                case ',': this.AddToken(TokenType.COMMA); break;
                case '.': this.AddToken(TokenType.DOT); break;
                case '-': this.AddToken(TokenType.MINUS); break;
                case '+': this.AddToken(TokenType.PLUS); break;
                case ';': this.AddToken(TokenType.SEMICOLON); break;
                case '*': this.AddToken(TokenType.STAR); break;

                default:
                    Lox.error(line, "Unexpected character.");
                    break;
            }
        }

        private char Advance()
        {
            this._current++;
            return this._source[this._current - 1];
        }

        private void AddToken(TokenType type)
        {
            this.AddToken(type, null);
        }

        private void AddToken(TokenType type, Object literal)
        {
            String text = this._source.Substring(this._start, this._current);
            this._tokens.Add(new Token(type, text, literal, this._line));
        }
    }
}
