# wasm-encoder-performance

Benchmarking [wasm-encoder](https://crates.io/crates/wasm-encoder) against an alternative implementation strategy.

## Results

On my 2020 M1 MacBook Pro, I see a roughly 30% difference in performance:

```
Encoding/Current/helpers
                        time:   [3.0009 µs 3.0142 µs 3.0280 µs]
                        change: [-3.6632% -2.0244% -0.6404%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
Encoding/Alternative/helpers
                        time:   [2.3175 µs 2.3325 µs 2.3486 µs]
                        change: [+0.7639% +2.0156% +3.0865%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
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

In [`src/encode.rs`](src/encode.rs) I've written a proof-of-concept implementation that instead splits each instruction encoding into its own function:

```rust
use wasm_encoder::{BlockType, Encode};

pub struct Fun {
    bytes: Vec<u8>,
}

impl Fun {
    fn encode(&mut self, x: impl Encode) {
        x.encode(&mut self.bytes);
    }

    pub fn unreachable(&mut self) {
        self.bytes.push(0x00);
    }

    pub fn nop(&mut self) {
        self.bytes.push(0x01);
    }

    pub fn block(&mut self, bt: BlockType) {
        self.bytes.push(0x02);
        self.encode(bt);
    }

    // ...
}
```

Example usage:

```rust
use crate::encode::Fun;

let mut f = Fun::new();
f.local_get(0);
f.local_get(1);
f.i32_add();
f.end();
```

As an aside, this also happens to result in more concise code.

## Usage

To reproduce, assuming you have [Rust](https://www.rust-lang.org/tools/install) and are using [Homebrew](https://brew.sh/):

```
brew install gnuplot
cargo bench
cargo install --locked resvg
resvg --zoom 3 --background white target/criterion/Encoding/report/violin.svg violin.png
```
