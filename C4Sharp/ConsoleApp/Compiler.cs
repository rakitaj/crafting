using Microsoft.VisualBasic.CompilerServices;
using System;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using System.Text;

namespace ConsoleApp
{
    public class Compiler
    {
        public string RawSource { get; set; }
        public int Line { get; set; }
        public int Position { get; set; }

        public Compiler(string source)
        {
            this.RawSource = source;

            this.Line = 1;
            this.Position = 0;
        }

        public bool IsAtEnd()
        {
            return this.RawSource.Length <= this.Position;
        }

        public List<Token> Lex()
        {
            var tokens = new List<Token>();
            
            while (!this.IsAtEnd())
            {
                var token = this.NextToken();
                if (token.GetType() != typeof(WhiteSpace))
                {
                    tokens.Add(token);
                }
            }
            return tokens;
        }

        public Token NextToken()
        {
            var current = this.RawSource[this.Position];
            Token token = null;
            if (char.IsWhiteSpace(current))
            {
                this.Position = this.Position + 1;
                return new WhiteSpace();
            }
            switch (current)
            {
                case '{':
                    token = new BraceOpen();
                    this.Position = this.Position + 1;
                    break;
                case '}':
                    token = new BraceClose();
                    this.Position = this.Position + 1;
                    break;
                case '(':
                    token = new ParenOpen();
                    this.Position = this.Position + 1;
                    break;
                case ')':
                    token = new ParenClose();
                    this.Position = this.Position + 1;
                    break;
                case ';':
                    token = new Semicolon();
                    this.Position = this.Position + 1;
                    break;
                default:
                    if (this.Position + 6 < this.RawSource.Length && this.RawSource.Substring(this.Position, 6).Equals("return"))
                    {
                        token = new KeywordReturn();
                        this.Position = this.Position + 6;
                    }
                    else if (this.Position + 3 < this.RawSource.Length && this.RawSource.Substring(this.Position, 3).Equals("int"))
                    {
                        token = new KeywordInt();
                        this.Position = this.Position + 3;
                    }
                    else if (char.IsDigit(current))
                    {
                        var match = LiteralInteger.Pattern.Match(this.RawSource, this.Position);
                        token = new LiteralInteger(match.Value);
                        this.Position = this.Position + match.Length;
                    }
                    else if (char.IsLetter(current))
                    {
                        var match = Identifier.Pattern.Match(this.RawSource, this.Position);
                        token = new Identifier(match.Value);
                        this.Position = this.Position + match.Length;
                    }
                    break;
            }
            return token;
        }
    }
}
