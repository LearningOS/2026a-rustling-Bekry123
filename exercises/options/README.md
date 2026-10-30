# Options

Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not.
Option types are very common in Rust code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on error
- Optional struct fields
- Struct fields that can be loaned or "taken"
- Optional function arguments
- Nullable pointers
- Swapping things out of difficult situations

## Further Information

- [Option Enum Format](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)

match可以处理多种情况，=>左侧是模式，右侧是匹配后要做的处理
match还可以当unwrap用，比如Some(x,y)=>Point，println!({x},{y})之类的。