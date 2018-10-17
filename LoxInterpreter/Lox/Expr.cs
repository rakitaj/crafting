using System;
using System.Collections.Generic;
using System.Text;

namespace Lox
{
    public abstract class Expr
    {

    }

    public class Binary: Expr
    {
        public readonly Expr Left;
        public readonly Token OperatorToken;
        public readonly Expr Right;

        public Binary(Expr left, Token operatorToken, Expr right)
        {
            this.Left = left;
            this.OperatorToken = operatorToken;
            this.Right = right;
        }
    }
}
