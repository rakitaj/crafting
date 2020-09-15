using System.Runtime.CompilerServices;
using System.Text.RegularExpressions;

namespace ConsoleApp
{
    public abstract class Token
    {
        public virtual string Value { get; protected set; }

        public override int GetHashCode()
        {
            return base.GetHashCode() * this.Value.GetHashCode();
        }

        public override bool Equals(object obj)
        {
            if (obj is Token)
            {
                Token other = obj as Token;
                return this.GetType() == other.GetType() && this.Value == other.Value;
            }
            else
            {
                return false;
            }
        }
    }

    public class BraceOpen : Token
    {
        public override string Value => "{";
    }

    public class BraceClose : Token
    {
        public override string Value => "}";
    }

    public class ParenOpen : Token
    {
        public override string Value => "(";
    }

    public class ParenClose : Token
    {
        public override string Value => ")";
    }

    public class Semicolon : Token
    {
        public override string Value => ";";
    }

    public class KeywordInt : Token
    {
        public override string Value => "int";
    }

    public class KeywordReturn : Token
    {
        public override string Value => "return";
    }

    public class Identifier : Token
    {
        public Identifier(string value)
        {
            this.Value = value;
        }

        public static Regex Pattern => new Regex("[a-zA-Z]\\w*");
    }

    public class LiteralInteger : Token
    {
        public LiteralInteger(string value)
        {
            this.Value = value;
        }

        public static Regex Pattern => new Regex("[0-9]+");
    }

    /// <summary>
    /// Special <see cref="WhiteSpace"/> token class that the lexer can discard.
    /// </summary>
    public class WhiteSpace : Token
    {
        public override string Value => string.Empty;
    }
}