wants:

- nominal typing
- union types, both anonymous and named
- arbitrary width integer types
- narrowing (for union types and integer types)
- type inference
- generics
- functions as first class types
- "const"/compile time code, basically a fully fleshed out version of rust's const evaluation; see also zig's comptime
- pattern matching
- some kind of type classes?

syntax:

- sum types defined with |, product types always defined with fields (similar to haskell); eunum variants must always be named; enum variants may be empty or use record syntax; anonymous enums defined with union keyword, variants are unnamed, nested unions "flatten" into each other (similar to typescript sum types):, see the section on inferred types for more information

  ```
  enum Enum = A(bool) | B | C { c: int }

  union Union = int | bool

  record Product = { field: bool }
  record TupleProduct = (bool)
  ```
- functions 
  ```
  fn f(a: int, b: SomeType) -> {}
  ```
- binary operators as you'd expect (a * b, etc) but force parenthesis because i'm lazy and don't feel like implementing [pratt parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
- type classes defined with the class keyword
  ```
  class Class<T> has
    fn static_method<U>(t: T, u: U) -> int;
    fn method_with_receiver(this, b: bool) -> T;
  ```
- generics
  ```
  union Either<L, R> = Left<L> | Right<R> 

  fn g<T: Add>(ts: List<T>) -> int {}
  ```
- type inference/ "deferring":

  TODO: Come up with a better syntax for inferred types? Maybe `open union`?
  
  ```
  # inferred to be 'int'
  fn f() -> _ {
    0
  }

  # inferred to be 'T | U'
  fn g<T, U>(b: bool, t: T, u: U) -> _ {
    if b then t else u
  }

  # inferred to be 'Result<int, ErrTooSmall | ErrTooLarge>'
  fn h(i: int) -> Result<int, _> {
    if i < 10
    then Err(ErrTooSmall)
    else if i > 20
    then Err(ErrTooLarge)
    else Ok(i)
  }

  # this type will be the union of all of the inferred types at all use sites
  union GloballyInferred = _
  # same as above, but with a few known variants
  union GloballyInferredWithDefaultVariants = TypeA | TypeB | _

  # inferred to be 'ErrorF | ErrorG'
  union Error = _

  fn f() -> Error {
    ErrorF
  }

  fn g() -> Error {
    ErrorG
  }

  # inferred unions flatten like normal unions:
  union A = int | _
  union B = bool | _
  # the following two types are equivalent:
  union C = A | B | _
  union C = int | bool | _

  # the result enum is defined like this:
  union Result<T, E> = Ok<T> | Err<E>
  record Ok<T>(T)
  record Err<E>(E)

  # and option like this:
  union Option<T> = None | Some<T>
  record None
  record Some<T>(T)

  # (note that Ok, Err, None, and Some are all standalone records)

  # union variants are unique, the following two types are equivalent:
  union MaybeAOrB = Option<A> | Option<B>
  union MaybeAOrB = None | Some<A> | Some<B>

  # an un-inferred generic, if unused, is a ZST
  # in this function, since the return type is specified as Option<_>, but the body only ever returns None, the return type of the function is inferred to only ever be None; the return type is thus Option<!>
  # this prevents needing to explicitly type unused params
  fn always_none() -> Option<_> {
    None
  }
  ```
- switch-case/match:
  haskell-ish syntax, rust-ish patterns
  ```
  case expr
    is Pattern(binding): {}
    catchall_binding: {}
  ```
