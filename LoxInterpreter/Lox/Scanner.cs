﻿using System;
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

                case '!': this.AddToken(this.Match('=') ? TokenType.BANG_EQUAL : TokenType.BANG); break;
                case '=': this.AddToken(this.Match('=') ? TokenType.EQUAL_EQUAL : TokenType.EQUAL); break;
                case '<': this.AddToken(this.Match('=') ? TokenType.LESS_EQUAL : TokenType.LESS); break;
                case '>': this.AddToken(this.Match('=') ? TokenType.GREATER_EQUAL : TokenType.GREATER); break;

                case '/':
                    if (this.Match('/'))
                    {
                        // A comment goes until the end of the line.                
                        while (this.Peek() != '\n' && !this.IsAtEnd()) this.Advance();
                    }
                    else
                    {
                        this.AddToken(TokenType.SLASH);
                    }
                    break;

                case ' ':
                case '\r':
                case '\t':
                    // Ignore whitespace.                      
                    break;

                case '\n':
                    this._line++;
                    break;

                case '"':
                    this.StringToken();
                    break;

                default:
                    if (IsDigit(c))
                    {
                        this.Number();
                    }
                    else
                    {
                        Program.Error(this._line, "Unexpected character.");
                    }
                    break;
            }
        }

        public static bool IsDigit(char c)
        {
            return c >= '0' && c <= '9';
        }

        private void Number()
        {
            while (IsDigit(this.Peek())) this.Advance();

            // Look for a fractional part.
            if (this.Peek() == '.' && IsDigit(this.PeekNext()))
            {
                // Consume the '.'
                this.Advance();

                while (IsDigit(this.Peek())) this.Advance();
            }

            this.AddToken(TokenType.NUMBER, double.Parse(this._source.Substring2(this._start, this._current)));
        }

        private char Peek()
        {
            if (this.IsAtEnd()) return '\0';
            return this._source[this._current];
        }

        private char PeekNext()
        {
            if (this._current + 1 >= this._source.Length) return '\0';
            return this._source[this._current + 1];
        }

        private bool Match(char expected)
        {
            if (this.IsAtEnd()) return false;
            if (this._source[this._current] != expected) return false;

            this._current++;
            return true;
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
            var length = this._current - this._start;
            String text = this._source.Substring(this._start, length);
            this._tokens.Add(new Token(type, text, literal, this._line));
        }

        private void StringToken()
        {
            while (this.Peek() != '"' && !this.IsAtEnd())
            {
                if (this.Peek() == '\n') this._line++;
                this.Advance();
            }

            if (this.IsAtEnd())
            {
                Program.Error(this._line, "Unterminated string.");
                return;
            }

            this.Advance();
            int length = (this._current - 1) - (this._start + 1);
            String value = this._source.Substring(this._start + 1, length);
            this.AddToken(TokenType.STRING, value);
        }
    }
}
