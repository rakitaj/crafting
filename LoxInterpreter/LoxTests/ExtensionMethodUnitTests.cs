using Lox;
using System;
using System.Collections.Generic;
using System.Text;
using Xunit;

namespace LoxTests
{
    public class ExtensionMethodUnitTests
    {
        [Theory]
        [InlineData("example", 0, 4, "exam")]
        [InlineData("example", 0, 0, "")]
        [InlineData("example", 0, 1, "e")]
        [InlineData("example", 1, 2, "x")]
        [InlineData("example", 1, 1, "")]
        [InlineData("foo", 0, 3, "foo")]
        public void Substring2_should_be_inclusive_for_start_exclusive_for_end(string source, int start, int end, string expectedResult)
        {
            var result = source.Substring2(start, end);
            Assert.Equal(expectedResult, result);
        }

        [Theory]
        [InlineData("foo", -1, 2)]
        [InlineData("foo", 2, 5)]
        [InlineData("foo", 3, 3)]
        [InlineData("foo", 0, 4)]
        public void Substring2_should_throw_if_start_or_end_are_out_of_bounds(string source, int start, int end)
        {
            Assert.Throws<ArgumentOutOfRangeException>(() =>
            {
                source.Substring2(start, end);
            });
        }
    }
}
