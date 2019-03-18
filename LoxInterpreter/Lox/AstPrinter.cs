//using System;
//using System.Collections.Generic;
//using System.Text;

//namespace Lox
//{
//    public class AstPrinter : IVisitor<String>
//    {
//        public string VisitBinaryExpr(Binary expr)
//        {
//            return this.Parenthesize(expr.operation.Lexeme, expr.left, expr.right);
//        }

//        public string VisitGroupingExpr(Grouping expr)
//        {
//            throw new NotImplementedException();
//        }

//        public string VisitLiteralExpr(Literal expr)
//        {
//            throw new NotImplementedException();
//        }

//        public string VisitUnaryExpr(Unary expr)
//        {
//            throw new NotImplementedException();
//        }

//        public string Parenthesize(string name, params Expr[] expressions)
//        {
//            StringBuilder builder = new StringBuilder();
//            builder.Append("(").Append(name);
//            foreach(Expr expr in expressions)
//            {
//                builder.Append("");
//                builder.Append(expr.Accept(this));
//            }
//            builder.Append(")");
//            return builder.ToString();
//        }
//    }
//}
