# constant_pool
A rust macro for making constant pools much like Java enums

# Usage
To define a constant pool, you must first define a struct or tuple struct type.

If you are defining a struct type, it must have a field of type `&str` called `name`. If it is a tuple struct, the first value must be of type `&str`.

Examples:

```rs
struct Color(&'static str, u8, u8, u8);
```

```rs
struct Team {
	name: &'static str,
  number: u8
}
```

Once your struct is defined, you can define the constant pool using the `constant_pool!` macro:

```rs
constant_pool! {
	pub Colors: Color {
    RED(255, 0, 0),
    BLUE(0, 0, 255),
    GREEN(0, 255, 0),
    YELLOW(0, 255, 255)
  }
}
```

```rs
constant_pool! {
  pub Teams: Team {
  	WHITE{number: 1},
    BLACK{number: 2},
    ORANGE{number: 3}
  }
}
```

The name of the constant pool is defined, followed by the struct type it will use. Once defined, it creates a namespace with a module for the constant pool, populating it with all the constants you defined. For the above examples, you could access `Color::RED` and `Team::WHITE` respectively.

Additionally, the constant pool generates a slice containing all of the constant values, under `Namespace::values`. For our examples, this will be `Color::values` and `Team::values`. It will also generate a function to get any of the values by name, returning an `Option<&T>`:

```rs
let color: &Color = Color::by_name("RED").unwrap();
let team = Team::by_name("ORANGE").unwrap();
```
