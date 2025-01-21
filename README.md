# wasm-encoder-performance

Benchmarking [wasm-encoder](https://crates.io/crates/wasm-encoder) against an alternative implementation strategy.

## Results

On my 2020 M1 MacBook Pro, I see a 28% difference in performance:

```
Encoding/Current/helpers
                        time:   [2.9529 µs 3.0171 µs 3.1023 µs]
                        change: [-1.9347% -0.7749% +0.6104%] (p = 0.27 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
Encoding/Alternative/helpers
                        time:   [2.3315 µs 2.3560 µs 2.3809 µs]
                        change: [-0.5493% +0.4016% +1.2946%] (p = 0.40 > 0.05)
                        No change in performance detected.
```

![violin plot](violin.png)

## Experiment

I pulled some [code to generate Wasm helper functions](https://github.com/samestep/floretta/blob/v0.3.0/crates/floretta/src/helper.rs) from a project of mine that uses wasm-encoder, reimplemented it using this alternative approach, and used [Criterion.rs](https://github.com/bheisler/criterion.rs) to compare the two.

## Overview

At time of writing, wasm-encoder is at [version 0.223.0](https://crates.io/crates/wasm-encoder/0.223.0). The crate documentation gives code examples that generally look like this, using the [`Function`](https://docs.rs/wasm-encoder/0.223.0/wasm_encoder/struct.Function.html) type:

```rust
use wasm_encoder::{Function, Instruction};

let mut f = Function::new([]);
f.instruction(&Instruction::LocalGet(0));
f.instruction(&Instruction::LocalGet(1));
f.instruction(&Instruction::I32Add);
f.instruction(&Instruction::End);
```

Another option is to just use the [`Encode`](https://docs.rs/wasm-encoder/0.223.0/wasm_encoder/trait.Encode.html) trait directly:

```rust
use wasm_encoder::{Encode, Instruction};

let mut sink = Vec::new();
Instruction::LocalGet(0).encode(&mut sink);
Instruction::LocalGet(1).encode(&mut sink);
Instruction::I32Add.encode(&mut sink);
Instruction::End.encode(&mut sink);
```

Either way, every time an instruction is encoded, it goes through [`impl Encode for Instruction<'_>`](https://github.com/bytecodealliance/wasm-tools/blob/v1.223.0/crates/wasm-encoder/src/core/code.rs#L1236-L1238), which is a big `match` expression:

```rust
impl Encode for Instruction<'_> {
    fn encode(&self, sink: &mut Vec<u8>) {
        match *self {
            // Control instructions.
            Instruction::Unreachable => sink.push(0x00),
            Instruction::Nop => sink.push(0x01),
            Instruction::Block(bt) => {
                sink.push(0x02);
                bt.encode(sink);
            }
            // ...
        }
    }
}
```

My _guess_ is that the compiler will generally not inline this function, so there's an extra branch for every instruction that gets encoded; and I'd also guess that this branch is not very predictable for modern CPUs. I'm not a performance engineering expert though, so please correct me if I'm guessing wrong here!

## Alternative

In [`src/encode.rs`](src/encode.rs) and [`src/sink.rs`](src/sink.rs) I've written a proof-of-concept implementation that replaces the `instruction` method with a `sink` method returning an `InstructionSink`, which encodes each diffeent instruction in its own function like this:

```rust
pub struct InstructionSink<'a> {
    sink: &'a mut Vec<u8>,
}

impl<'a> InstructionSink<'a> {
    pub fn new(sink: &'a mut Vec<u8>) -> Self {
        Self { sink }
    }

    // Control instructions.

    pub fn unreachable(&mut self) -> &mut Self {
        self.sink.push(0x00);
        self
    }

    pub fn nop(&mut self) -> &mut Self {
        self.sink.push(0x01);
        self
    }

    pub fn block(&mut self, bt: BlockType) -> &mut Self {
        self.sink.push(0x02);
        bt.encode(self.sink);
        self
    }

    // ...
}
```

Example usage:

```rust
use crate::encode::Fun;

let mut f = Fun::new();
f.sink()
    .local_get(0)
    .local_get(1)
    .i32_add()
    .end();
```

As an aside, this also happens to result in more concise code.

The contents of `src/sink.rs` are generated using a [script](src/main.rs) that parses [`src/code.rs`](src/code.rs), which itself is copied from [`src/core/code.rs` in the wasm-tools repo](https://github.com/bytecodealliance/wasm-tools/blob/6e9164c5db03892c9dc603ba1f783e84d0eacdd7/crates/wasm-encoder/src/core/code.rs#L1239-L3783). You can run it like this:

```sh
brew install sponge
cargo run | sponge src/sink.rs
```

## Usage

To reproduce, assuming you have [Rust](https://www.rust-lang.org/tools/install) and are using [Homebrew](https://brew.sh/):

```
brew install gnuplot
cargo bench
cargo install --locked resvg
resvg --zoom 3 --background white target/criterion/Encoding/report/violin.svg violin.png
```
