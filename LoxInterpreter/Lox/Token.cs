using System;
using System.Collections.Generic;
using System.Text;

namespace Lox
{
    public class Token
    {
        public readonly TokenType Type;
        public readonly String Lexeme;
        public readonly Object Literal;
        public readonly int Line;

        public Token(TokenType type, String lexeme, Object literal, int line)
        {
            this.Type = type;
            this.Lexeme = lexeme;
            this.Literal = literal;
            this.Line = line;
        }

        public override int GetHashCode()
        {
            return this.Line.GetHashCode()
                ^ this.Type.GetHashCode()
                ^ this.Lexeme.GetHashCode()
                ^ this.Literal.GetHashCode();
        }

        public override String ToString()
        {
            return this.Line + " " + this.Type + " " + this.Lexeme + " " + this.Literal;
        }

        public override bool Equals(object obj)
        {
            var other = obj as Token;

            if (other == null)
            {
                return false;
            }


            bool literalEqual = false;
            if (other.Type == TokenType.NUMBER)
            {
                literalEqual = (double)this.Literal == (double)other.Literal;
            }
            else if (other.Type == TokenType.STRING)
            {
                literalEqual = (string)this.Literal == (string)other.Literal;
            }
            else
            {
                literalEqual = this.Literal == other.Literal;
            }

            return this.Type == other.Type
                && this.Lexeme == other.Lexeme
                && literalEqual
                && this.Line == other.Line;
        }

    }
}
