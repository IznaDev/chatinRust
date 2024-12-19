# chatinRust
A decentralised App for chating.

In this project, I developed a simple & basic decentralised chat with Rust and WebAssembly. It can be deployed on Wavelet bockchain.

There are two tests, you can run it with the instruction cargo test.

to build the lib is not as usual because of the WebAssembly Compiler, mandatory to compile a rust code for web browsers. So you have to run this instruction : cargo build --release --target wasm32-unknown-unknown

--release : it's the building mode release to optimize the building

--target : it's the building target i.e. the lib built is wasm32 target compliant.

wasm32-unknown-unknown : it is the target, web assambly in 32 bits unknown vendor (architecture) and unknown OS.
