# WASM BPE Tokenizer

A high-performance Byte-Pair Encoding (BPE) tokenizer written in pure Rust and compiled to WebAssembly (WASM). While Python is the standard for AI orchestration, text tokenization is fundamentally a string-manipulation and search problem. Relying on Python for this step creates a severe bottleneck, especially in edge-computing scenarios. This project bypasses that bottleneck entirely, allowing the tokenizer to run instantly and natively inside any modern web browser or edge runtime, shifting the compute load away from the server and enabling zero-latency text preprocessing before the data ever reaches the LLM inference gateway.

## Core Architecture

* **Zero-Dependency Engine:** Implements the core BPE merge logic in pure Rust to achieve C-level execution speeds and memory safety without relying on external Python bloat.
* **WASM Bridge:** Utilizes `wasm-bindgen` to create seamless interoperability between the compiled Rust binary and JavaScript/TypeScript frontends.
* **Edge-Native Design:** Engineered specifically to bypass server-side processing, allowing the tokenizer to execute directly on the user's local machine or distributed serverless networks like Cloudflare Workers.

## Tech Stack

* **Language:** Rust, WebAssembly (WASM)
* **Architecture:** Edge Computing, C-Level Memory Management
* **Domain:** AI Data Infrastructure, Large Language Models (LLMs)

## Build and Execution

This project requires the Rust compiler (`rustc`) and `wasm-pack` to be installed on your system.

```bash
# 1. Clone the repository
git clone [https://github.com/nolanefe/wasm-bpe-tokenizer.git](https://github.com/nolanefe/wasm-bpe-tokenizer.git)
cd wasm-bpe-tokenizer

# 2. Compile the Rust code to WebAssembly
wasm-pack build --target web