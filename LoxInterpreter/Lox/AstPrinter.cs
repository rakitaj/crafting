using System;
using System.Collections.Generic;
using System.Text;

namespace Lox
{
    public class AstPrinter : IVisitor<String>
    {
        public String Print(Expr expr)
        {
            return expr.Accept(this);
        }

        public string VisitBinaryExpr(Binary expr)
        {
            return this.Parenthesize(expr.operation.Lexeme, expr.left, expr.right);
        }

        public string VisitGroupingExpr(Grouping expr)
        {
            return this.Parenthesize("group", expr.expression);
        }

        public string VisitLiteralExpr(Literal expr)
        {
            if (expr.value == null)
            {
                return "nil";
            }
            else
            {
                return expr.value.ToString();
            }
        }

        public string VisitUnaryExpr(Unary expr)
        {
            return this.Parenthesize(expr.operation.Lexeme, expr.right);
        }

        public string Parenthesize(string name, params Expr[] expressions)
        {
            StringBuilder builder = new StringBuilder();
            builder.Append("(").Append(name);
            foreach (Expr expr in expressions)
            {
                builder.Append(" ");
                builder.Append(expr.Accept(this));
            }
            builder.Append(")");
            return builder.ToString();
        }
    }
}
