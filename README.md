# Call Rust functions from Ruby using Docker

Based on the tutorial [Rust to the rescue (of Ruby)](https://fbzga.medium.com/rust-to-the-rescue-of-ruby-2067f5e1dc25)

## Directory structure

The root directory contains a Dockerfile, a Ruby project, and a Cargo package.

```
.
├── docker-compose.yml
├── Dockerfile
├── fibonacci
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── ruby
    ├── Gemfile
    ├── Gemfile.lock
    └── main.rb
```

## Cargo.toml

Instructs Cargo to compile the package as a dylib

```toml
# ...
[lib]
name = "fibonacci"
crate-type = ["dylib"]
# ...
```

## src/lib.rs

The function to be called from Ruby

```rust
#[no_mangle]
pub extern fn fibonacci(n: u32) -> u32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

## Dockerfile

The Dockerfile uses a multi-stage build to compile the Cargo package, and copies the `target/release` directory into the Ruby container image as `/fibonacci`.

```Dockerfile
FROM rust:latest
WORKDIR /code
COPY fibonacci/ /code/
RUN cargo build --release

FROM ruby:3.0.2
WORKDIR /code
COPY ruby/Gemfile ruby/Gemfile.lock /code/
RUN bundle install
COPY --from=0 /code/target/release /fibonacci
COPY ruby/ /code/
CMD ["bundle", "exec", "ruby", "main.rb"]
```

## main.rb

Uses the ffi gem to load the dynamic library built by Cargo and call the `fibonacci` function

```ruby
require 'ffi'

module ExternalFunctions
  extend FFI::Library
  ffi_lib '/fibonacci/libfibonacci.so'
  attach_function :fibonacci, [:int], :int
end

puts ExternalFunctions.fibonacci(42)
```

## Troubleshooting

- **`LoadError (Could not open library '/fibonacci/libfibonacci.so': Error loading shared library ld-linux-x86-64.so.2: No such file or directory (needed by /fibonacci/libfibonacci.so))`**

  Caused by using an Alpine version of the Ruby base image 

- **`error: cannot produce dylib for 'fibonacci v0.1.0 (/code)' as the target 'x86_64-unknown-linux-musl' does not support these crate types`**

  Caused by using an Alpine version of the Rust base image
