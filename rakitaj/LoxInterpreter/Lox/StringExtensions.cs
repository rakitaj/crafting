using System;
using System.Collections.Generic;
using System.Text;

namespace Lox
{
    public static class StringExtensions
    {
        public static string Substring2(this string str, int start, int end)
        {
            if (start > str.Length - 1)
            {
                throw new ArgumentOutOfRangeException(nameof(start), "Must be within the bounds of the string length.");
            }
            int length = end - start;
            return str.Substring(start, length);
        }
    }
}
