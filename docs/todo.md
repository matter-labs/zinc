## Todo

### Vars

- linear combination optimizations
- var naming
- var name scoping
- vector methods
- `vector<bool, size>.pack()`
- type inference
- type conversions

### State

- contract variables

### Unsorted

- modular division, invesrsion
- exponentiation
- debug trace
- bitshift ops
- recursion

### Bellman gadgets

- sha256
- pedersen
- sig_verify

### Code conversion samples

- [ ] inputs: `inputs { a: type, b: type }`
- [ ] witness: `witness { a: type, b: type }`
- [ ] witness generators: `unsafe_witness { /* bellman/rust code */ }`
- [ ] types: `let [mut] a: {type} = {value};`
- [ ] operators: LCs, ranges, overflow checks => range check on assignment
- [ ] require: `require({boolean condition});`
- [ ] if: conditional assignments, computational reuse optimizations
- [ ] for: constant range
- [ ] struct: assignments
- [ ] functions / gadgets
- [ ] unsafe_witness{} code

### Optimizations

- [ ] conditional accumulation of heavy functions

### Formal

- [ ] Formal language spec

### Vectors

- `memory_vector<T, size>`: fixed-sized array of elements of a given type in memory
- `storage_vector<T, size>`: fixed-sized array of elements of a given type in storage (tbd)

__Implementation details:__ vectors with random index access can have different implementations depending on the vector size and the way it is used. Possible implementations:

- Merkle tree
- Linear scan

### Supported operators for vectors

- `[c]`: access element by index `c` (where `c` is a constant)
- `[i]`: access element by index `i` (where `i` is a integer variable)

**Embedded methods**:

- `into_bits()`: yields `memory_array<bool, bit_length>`