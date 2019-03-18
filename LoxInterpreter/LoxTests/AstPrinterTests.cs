using Lox;
using System;
using System.Collections.Generic;
using System.Text;
using Xunit;

namespace LoxTests
{
    public class AstPrinterTests
    {
        [Trait("Category", "Unit")]
        [Fact]
        public void PrettyPrint_SanityCheck_From_Book()
        {
            Expr expression = new Binary(
        new Unary(
            new Token(TokenType.MINUS, "-", null, 1),
            new Literal(123)),
        new Token(TokenType.STAR, "*", null, 1),
        new Grouping(
            new Literal(45.67)));

            Assert.Equal("(* (- 123) (group 45.67))", new AstPrinter().Print(expression));
        }
    }
}
