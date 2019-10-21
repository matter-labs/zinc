# String

No known differences from the Rust behavior, taking into account its very
limited implementation and usability.

The string type exists only in the literal form and can only appear as the
second argument of the `require` statement.

## Examples

```zrs
require(true != false, "mega ultra extra total global example");
```
