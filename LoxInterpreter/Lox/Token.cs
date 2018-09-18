using System;
using System.Collections.Generic;
using System.Text;

namespace Lox
{
    public class Token
    {
        public string Value { get; private set; }

        public Token(string value)
        {
            this.Value = value;
        }

        public override string ToString()
        {
            return this.Value;
        }
    }
}
