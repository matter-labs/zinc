# r1cs constraints

## Add

Expression:

    c = a + b

R1CS:

    (a + b) * (1) = (c)

## Sub

Expression:

    c = a - b

R1CS:

    (a - b) * (1) = (c)

## Mul

Expression:

    c = a * b

R1CS:

    (a) * (b) = (c)

## Div & Mod

Integer division of `N`-bit numbers. Fraction part is discarded.
Variables are nominator (`n`), denominator (`d`), quotient (`q`), and remainder (`r`).

Expression:

    q = n / d
    r = n % d
    
We will transform expression to `n = qd + r` with constraint `r < d`.

R1CS:

    // Enforce n = qd + r
    (q) * (d) = (n - r)

    // Enforce `r < d`
    (bit0 * 2^0 + bit1 * 2^1 + ... + bitN-1 * 2^N-1) * (1) = (d - r)
    (bit0) * (1 - bit0) = (0)
    (bit1) * (1 - bit1) = (0)
    ...
    (bitN) * (1 - bitN) = (0)

## And

Expression:

    c = a AND b
    
R1CS:

    (a) * (b) = (c)
    
## Or

Expression:

    c = a OR b
    
R1CS:

    (1 - a) * (1 - b) = (1 - c)
    
## Xor

Expression:

    c = a XOR b
    
R1CS:

    (a + a) * (b) = (a + b - c)

## Not

Expression:

    b = NOT a
    
R1CS:

    (1) * (1) = (a + b)
