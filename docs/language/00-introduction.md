# Introduction

The Jabberwocky language is used to simplify development of Quadratic Arithmetic
Programs (see [this example](http://coders-errand.com/how-to-build-a-quadratic-arithmetic-program/)).
Its transpiler converts a program into an R1CS circuit using the
[bellman](https://github.com/matter-labs/bellman)
library. This allows generation of Zero Knowledge Proofs for any proof system
supported by bellman (such as Groth16 or Sonic).
