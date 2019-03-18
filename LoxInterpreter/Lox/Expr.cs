using System;
using System.Collections.Generic;
namespace Lox
{
public abstract class Expr {
}
public class Binary : Expr {
  public Binary(Expr left, Token operation, Expr right) {
      this.left = left;
      this.operation = operation;
      this.right = right;
    }

    public readonly Expr left;
    public readonly Token operation;
    public readonly Expr right;
  }
public class Grouping : Expr {
  public Grouping(Expr expression) {
      this.expression = expression;
    }

    public readonly Expr expression;
  }
public class Literal : Expr {
  public Literal(Object value) {
      this.value = value;
    }

    public readonly Object value;
  }
public class Unary : Expr {
  public Unary(Token operation, Expr right) {
      this.operation = operation;
      this.right = right;
    }

    public readonly Token operation;
    public readonly Expr right;
  }
}
