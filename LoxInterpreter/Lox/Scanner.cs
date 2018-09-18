using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Lox
{
    public class Scanner
    {
        public string Text { get; private set; }

        public Scanner(string text)
        {
            this.Text = text;
        }

        public List<Token> ScanTokens()
        {
            var splits = this.Text.Split(new char[] { ' ' });
            return this.SplitsToTokens(splits);
        }

        public List<Token> SplitsToTokens(IList<string> splits)
        {
            return splits.Select(split => new Token(split)).ToList();
        }
    }
}
