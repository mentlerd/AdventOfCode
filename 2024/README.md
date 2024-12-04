### 2024 - Journal

Trying Rust after all the memeing @ work. Using RustRover as an IDE for now.

### Day 01 - Setup

Pretty standard setup, I have followed the [recommended Rust installation instructions][1], and ended up 
with a working `rustup` within minutes. I picked [RustRover][2] as my IDE of choice as I am fairly familiar
with JetBrains products from my JVM days :)

RustRover's "LLDB renderers" seem [borked out-of-the box][7] with `rustc 1.83.0`. Luckily this area is one of my
~~obsessions~~ interests, thus I am fairly familiar with LLDB's [scriptable variable formatting capabilities][3] 
to the point that I have created [some of my own][4] for [libcxx][5] and Google's [Abseil][6].

While scripting LLDB is incredibly fun, I ended up downgrading the installed toolchain to a version which works
with RustRover out-of-the box instead of trying to fix the problem:
```shell
rustup default 1.81.0
```

On the topic of the task there isn't much to talk about. I found Rust's type inference quite impressive so far!

[1]: https://www.rust-lang.org/tools/install
[2]: https://www.jetbrains.com/rust/
[3]: https://lldb.llvm.org/use/variable.html#synthetic-children
[4]: https://github.com/mentlerd/lldb-toybox
[5]: https://github.com/llvm/llvm-project/tree/main/libcxx
[6]: https://github.com/abseil/abseil-cpp
[7]: https://youtrack.jetbrains.com/issue/RUST-15839/Strings-and-vecs-dont-show-in-the-debugger

### Day 03 - Interpreter

It's been a while since I had to do any sort of manual text parsing. Implementing a simple greedy interpreter
seemed both nostalgic and fun to do. Associated values in enums is great!

I have to admit that I mostly just powered through most of the excercises so far without giving a proper read
to the rust book.. hopefully this defficiency does not catch up with be before the weekend :)

### Day 04 - Rotating vertical detectors

I have been always fascinated by how spatial geometry equations and problems can be made orders of magnitudes 
easier by just transforming the input into a convenient local coordinate system. Today's task is no different,
where the pesky diagonals can be made trivial to reason about by skewing the input data in the negative/positive
direction by "moving" the detector state machines around.

An other neat trick of doing this is to do one more perspective shift. Why spend O(N) to actually move the detectors
when you can just move your view instead in O(1) using circular buffers :)
