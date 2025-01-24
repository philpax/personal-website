+++
title = "Abusing The C Preprocessor: Writing A 4-Bit Adder"
short = "String Interpolation Is All You Need."
date = 2013-10-01

[taxonomies]
tags=["c", "cursed"]
+++

I've always wondered what could be done with the C preprocessor. The existence of projects such as [Boost Preprocessor](http://www.boost.org/doc/libs/release/libs/preprocessor/) and the infamous [Brainfuck interpreter](https://github.com/orangeduck/CPP_COMPLETE) are a testament to its wide-ranging capabilities.

Of course, the best way to learn is to do, and I felt like indulging in some masochism. So - onwards!

<!-- more -->

Token pasting is the main (see: *only*) operation in the C preprocessor. Two tokens (input strings) are added together (concatenated); while this seems rather limited, you'll see it has plenty of potential:

```c
#include <stdio.h>

/* The token pasting abuse that makes this possible */
#define JOIN_INTERNAL( a, b ) a##b
#define JOIN( a, b ) JOIN_INTERNAL( a, b )
```

We use two macros here because our main `JOIN` macro needs to expand its arguments (which will be macros themselves!) before passing it onto `JOIN_INTERNAL` - otherwise, the macros will go through without being expanded, which will not produce the desired results.

# Binary Operators

## XOR

```c
/* Operators */
/* XOR */
#define XOR_00 0
#define XOR_10 1
#define XOR_01 1
#define XOR_11 0

#define XOR( a, b ) JOIN( XOR_, JOIN( a, b ) )
```

## AND

```c
/* AND */
#define AND_00 0
#define AND_10 0
#define AND_01 0
#define AND_11 1

#define AND( a, b ) JOIN( AND_, JOIN( a, b ) )
```

Now, we have token pasting. We know that we can add together tokens to produce new tokens. Say we have two tokens, representing bits: `0` and `1`. If we token paste them together, we get `01`. Now, if we token paste *that* with another token, say `XOR_`, we can get `XOR_01`, which is another token.

We know that the preprocessor is all too happy to expand these - so what if we made a [truth table](http://en.wikipedia.org/wiki/Truth_table) of tokens? By defining macros which correspond to our tokens (`XOR_00`, `XOR_01`, etc), the values of these macros are substituted when we token paste `XOR_`, `0`, and `1` together - we have created an operator!

By defining the truth table for XOR and AND, we've set up the basic building blocks that we need for our 4-bit adder.

# Full Adder

```c
/* Full adder macros */
/* Out = (A ^ B) ^ cin */
#define FULL_ADD_OUT( a, b, cin ) \
    XOR( XOR( a, b ), cin )

/* Carry_out = (A & B) ^ (Carry_in & (A ^ B)) */
/* The standard adder uses OR for the last 'gate' - this can safely be changed
   to XOR, which has been done here to avoid defining an OR operator */
#define FULL_ADD_CARRY( a, b, cin ) \
    XOR( AND( XOR( a, b ), cin ), AND( a, b ) )
```

Here lies the core of the adder. These macros transcode the logic behind the [full adder](http://en.wikipedia.org/wiki/Adder_(electronics)#Full_adder) - the equations have been reproduced in the comments.

These macros basically chain together the `XOR` and `AND` operators we made before, so that we can calculate the output and `Carry Out` bits from our input (the `A`, `B` and `Carry In` bits).

To calculate an addition, we call `FULL_ADD_OUT` and `FULL_ADD_CARRY` for each output bit; the parameters are the corresponding input bits, and the `Carry Out` from the last bit. Basically, we chain each full adder by its `Carry Out` output. For the first full adder, we simply pass 0 into `Carry In`.

# Conversion

Binary numbers are a convenient representation for computational arithmetic, as we can see in our implementation of the full adder. Unfortunately, they're slightly less convenient for humans and compilers expecting human input. With some clever token pasting trickery, we can define conversion operators:

```c
/* Number -> binary conversion */
#define NUMBER_TO_BINARY_0 0, 0, 0, 0
#define NUMBER_TO_BINARY_1 0, 0, 0, 1
#define NUMBER_TO_BINARY_2 0, 0, 1, 0
#define NUMBER_TO_BINARY_3 0, 0, 1, 1
/* ... To NUMBER_TO_BINARY_15 ... */

#define NUMBER_TO_BINARY( a ) JOIN( NUMBER_TO_BINARY_, a )

/* Binary -> number conversion */
#define BINARY_TO_NUMBER_0000 0
#define BINARY_TO_NUMBER_0001 1
#define BINARY_TO_NUMBER_0010 2
#define BINARY_TO_NUMBER_0011 3
/* ... To BINARY_TO_NUMBER_1111 ... */

#define BINARY_TO_NUMBER_INTERNAL( a, b, c, d ) \
    JOIN( BINARY_TO_NUMBER_, JOIN( a, JOIN( b, JOIN( c, d ) ) ) )

/* This is necessary to expand an incoming macro into its binary
   form; otherwise, only one argument will be passed into the macro
   expecting four */
#define BINARY_TO_NUMBER( x ) BINARY_TO_NUMBER_INTERNAL( x )
```

`NUMBER_TO_BINARY` takes in a base-10 number from 0 to 15, adds it to `NUMBER_TO_BINARY_`, and then returns the appropriate token; this expands to the binary format that our adder uses.

`BINARY_TO_NUMBER` takes in four bits, pastes them together (`0, 1, 1, 0` becomes `0110`), and then pastes that with `BINARY_TO_NUMBER_`, which expands to a normal base-10 number.

Of note is that there are two macros - `BINARY_TO_NUMBER_INTERNAL`, and `BINARY_TO_NUMBER`. You see, this macro suffers from the same problem as `JOIN` - it needs to expand its arguments before we can use it! `BINARY_TO_NUMBER` simply expands the arguments and passes it to `BINARY_TO_NUMBER_INTERNAL`, which produces the correct token.

# Four-Bit Addition

```c
/* Four-bit addition! Can easily be extended, but gets messy for
   obvious reasons */
#define ADD_A_B_4BIT( a1, a2, a3, a4, b1, b2, b3, b4 ) \
    FULL_ADD_OUT( a1, b1, \
        FULL_ADD_CARRY( a2, b2, \
            FULL_ADD_CARRY( a3, b3, \
                FULL_ADD_CARRY( a4, b4, 0 ) ) ) ), \
    \
    FULL_ADD_OUT( a2, b2, \
        FULL_ADD_CARRY( a3, b3, \
            FULL_ADD_CARRY( a4, b4, 0 ) ) ), \
    \
    FULL_ADD_OUT( a3, b3, \
        FULL_ADD_CARRY( a4, b4, 0 ) ), \
    \
    FULL_ADD_OUT( a4, b4, 0 ) \

/* This macro receives two NUMBER macros, which expand to the 4-bits necessary
   for each argument */
#define ADD_A_B_4BIT_NUMBERS( n1, n2 ) ADD_A_B_4BIT( n1, n2 )
```

And now: four-bit addition. Looks nasty? That's because it is. This macro spits out four bits, just like our `NUMBER_TO_BINARY` macros.

Each bit is the result of `FULL_ADD_OUT`; as there's no way to save the `FULL_ADD_CARRY` from each bit, we deal with this problem in a questionable manner: we manually nest the `FULL_ADD_CARRY` macros. The last bit has no carry, so we set it to zero; the second-last bit has the carry from the last bit; the third-last bit has the carry from the second-last bit, which is itself defined in terms of the last bit's carry; and so on, until we reach the first bit.

After unpacking, we can see that this is a rather simple process that results in the 4-bit addition of the numbers we pass in. We have to wrap `ADD_A_B_4BIT` in `ADD_A_B_4BIT_NUMBERS` so that macros that we pass in, such as the `NUMBER` macros, get converted to binary. Phew!

# Testing

```c
int main()
{
    printf( "%i + %i = %i\n",
        NUMBER_1,
        NUMBER_2,
        BINARY_TO_NUMBER(
            ADD_A_B_4BIT_NUMBERS(
                NUMBER_TO_BINARY( NUMBER_1 ),
                NUMBER_TO_BINARY( NUMBER_2 ) ) ) );
}
```

For the conclusion, our test case. We have our standard `int main()` function, and a seemingly standard `printf` showing an addition. But wait! In lieu of actual numbers, we have `NUMBER_1`, `NUMBER_2`, and our addition macro.

`NUMBER_1` and `NUMBER_2` aren't defined in the actual code: we provide them as a compile-time argument (via `-D`). This code is relatively simple: our two numbers are converted to binary, run through our addition macro, and the result is converted back into a number, just in time to be passed into our `printf`.

Time to test this sucker!

```bash
philpax@morningtide:$ clang full_add.c -DNUMBER_1=7 -DNUMBER_2=4
philpax@morningtide:$ ./a.out
7 + 4 = 11
```

Ain't she a beaut?

And to round things out, here's the expanded version of the `int main()` function after the C preprocessor has run.

```bash
philpax@morningtide:$ clang -E full_add.c -DNUMBER_1=7 -DNUMBER_2=4
```
```c
/* Omitted for brevity */
int main()
{
    printf( "%i + %i = %i\n",
        7,
        4,
        11 );
}
```

I hope you've enjoyed this brief look at the mysterious, powerful, and quite frankly *insane* C preprocessor. A compiling version of the full code can be found [here](https://github.com/Philpax/binarycpp/tree/ec87c09088eacc2b95ff7a8b35093cb0176afdd6). At the time of testing, this code worked in GCC and Clang, but not Visual C++.
