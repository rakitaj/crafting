using ConsoleApp;
using FluentAssertions;
using Xunit;

namespace C4Tests
{
    public class TokenTests
    {
        [Fact]
        public void Test_Two_Tokens_Same_Value_And_Type_Should_Be_Equal()
        {
            var token1 = new Identifier("main");
            var token2 = new Identifier("main");
            token1.Should().Be(token2);
        }

        [Fact]
        public void Test_Two_Tokens_Same_Value_Different_Type_Should_Not_Be_Equal()
        {
            var token1 = new Identifier(";");
            var token2 = new Semicolon();
            token1.Should().NotBe(token2);
        }

        [Fact]
        public void Test_Two_Tokens_Different_Value_Same_Type_Should_Not_Be_Equal()
        {
            var token1 = new LiteralInteger("3");
            var token2 = new LiteralInteger("5");
            token1.Should().NotBe(token2);
        }
    }
}
