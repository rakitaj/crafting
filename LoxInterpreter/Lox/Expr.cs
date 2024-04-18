using System;
using System.Collections.Generic;
namespace Lox
{
    public abstract class Expr
    {
        public abstract R Accept<R>(IVisitor<R> visitor);
    }
    public interface IVisitor<R>
    {
        R VisitBinaryExpr(Binary expr);
        R VisitGroupingExpr(Grouping expr);
        R VisitLiteralExpr(Literal expr);
        R VisitUnaryExpr(Unary expr);
    }
    public class Binary : Expr
    {
        public Binary(Expr left, Token operation, Expr right)
        {
            this.left = left;
            this.operation = operation;
            this.right = right;
        }

        public override R Accept<R>(IVisitor<R> visitor)
        {
            return visitor.VisitBinaryExpr(this);
        }

        public readonly Expr left;
        public readonly Token operation;
        public readonly Expr right;
    }
    public class Grouping : Expr
    {
        public Grouping(Expr expression)
        {
            this.expression = expression;
        }

        public override R Accept<R>(IVisitor<R> visitor)
        {
            return visitor.VisitGroupingExpr(this);
        }

        public readonly Expr expression;
    }
    public class Literal : Expr
    {
        public Literal(Object value)
        {
            this.value = value;
        }

        public override R Accept<R>(IVisitor<R> visitor)
        {
            return visitor.VisitLiteralExpr(this);
        }

        public readonly Object value;
    }
    public class Unary : Expr
    {
        public Unary(Token operation, Expr right)
        {
            this.operation = operation;
            this.right = right;
        }

        public override R Accept<R>(IVisitor<R> visitor)
        {
            return visitor.VisitUnaryExpr(this);
        }

        public readonly Token operation;
        public readonly Expr right;
    }
}
