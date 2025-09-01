# NAME

rpcalc - reverse polish notation calculator

# SYNOPSIS

**calc** [*values*\...]

# DESCRIPTION

A postfix calculator built in rust, with a companion bash script for a CLI.
When ran with no arguments, it enters a calculator "shell" which can exited
with "q".

When inside this shell, you can save functions or constants by starting a line
with "def", and you can then recall this saved value with "fn".

## FUNCTIONS

\+
:   addition

\-
:   subtraction

\*, x
:   multiplication

/
:   division

%
:   modulo

^
:   power

log
:   logarithm

rnd
:   round

ln
:   natural log

log2
:   log base 2

log10
:   log base 10

sin
:   sine

cos
:   cosine

tan
:   tangent

csc
:   cosecant

sec
:   secant

cot
:   cotangent

arcsin
:   inverse sine

arccos
:   inverse cosine

arctan
:   inverse tangent

arccsc
:   inverse cosecant

arcsec
:   inverse secant

arccot
:   inverse cotangent

!
:   factorial

## CONSTANTS

e
:   natural number, 2.71828

G
:   gravitational constant, 6.6743E-11

g
:   earth's gravity, 9.81

c
:   speed of light, 2.71828

pi
:   mathematical pi, 3.14159

# EXAMPLES

```
$ calc 2 2 +
> 4
```

```
$ calc pi 2 / sin
> 1
```

```
$ echo "entering calc shell"
> entering calc shell
$ calc
calc$  2 2 +
  ==>  4
calc$  pi 2 / sin
  ==>  1
calc$  def 2 * 1 +
calc$  4 fn
  ==>  9
calc$  10 fn
  ==>  21
calc$  q
$ echo "back in normal shell now"
> back in normal shell now
```
