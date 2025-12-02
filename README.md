# C2RUSTAGENT

AI agent that translates C code to Rust, compiles and verifies it works.

## Setup

```bash
# 1. Create environment
conda env create -f environment.yml
conda activate rustagent

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Set API key
export ANTHROPIC_API_KEY="your-key"
```

## Usage

**Single file:**
```bash
python c_to_rust_agent.py hello.c
python c_to_rust_agent.py hello.c --no-main
```

**Batch translate:**
```bash
python c_to_rust_agent.py --auto ./c_files ./rust_output
```