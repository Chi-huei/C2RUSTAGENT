#!/bin/bash
# setup.sh - Complete setup script for C to Rust Translation Agent
# Run with: bash setup.sh

set -e

echo "═══════════════════════════════════════════════════════════════"
echo "        C to Rust Translation Agent - Setup Script              "
echo "═══════════════════════════════════════════════════════════════"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if conda is installed
if ! command -v conda &> /dev/null; then
    echo -e "${RED}Error: conda is not installed.${NC}"
    echo "Please install Miniconda or Anaconda first:"
    echo "  https://docs.conda.io/en/latest/miniconda.html"
    exit 1
fi

echo -e "${GREEN}✓ Conda found${NC}"

# Create conda environment
echo
echo "Creating conda environment 'rustagent'..."
if conda env list | grep -q "^rustagent "; then
    echo -e "${YELLOW}Environment 'rustagent' already exists. Updating...${NC}"
    conda env update -n rustagent -f environment.yml --prune
else
    conda env create -f environment.yml
fi
echo -e "${GREEN}✓ Conda environment ready${NC}"

# Check if Rust is installed
echo
echo "Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
    echo -e "${YELLOW}Rust not found. Installing via rustup...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi
echo -e "${GREEN}✓ Rust $(rustc --version) installed${NC}"

# Create .env file if it doesn't exist
echo
if [ ! -f .env ]; then
    echo "Creating .env file..."
    cat > .env << 'EOF'
# Anthropic API Key
# Get your key at: https://console.anthropic.com/
ANTHROPIC_API_KEY=your-api-key-here
EOF
    echo -e "${YELLOW}⚠ Created .env file - please add your ANTHROPIC_API_KEY${NC}"
else
    echo -e "${GREEN}✓ .env file exists${NC}"
fi

# Create example C file
echo
if [ ! -f example.c ]; then
    echo "Creating example.c..."
    cat > example.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>

// Calculate factorial recursively
int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

// Calculate fibonacci iteratively
int fibonacci(int n) {
    if (n <= 1) return n;
    int a = 0, b = 1, temp;
    for (int i = 2; i <= n; i++) {
        temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

int main() {
    printf("Factorial and Fibonacci Calculator\n");
    printf("===================================\n\n");
    
    for (int i = 0; i <= 10; i++) {
        printf("factorial(%2d) = %7d    fibonacci(%2d) = %3d\n", 
               i, factorial(i), i, fibonacci(i));
    }
    
    return 0;
}
EOF
    echo -e "${GREEN}✓ Created example.c${NC}"
fi

echo
echo "═══════════════════════════════════════════════════════════════"
echo -e "${GREEN}Setup complete!${NC}"
echo "═══════════════════════════════════════════════════════════════"
echo
echo "Next steps:"
echo
echo "  1. Add your API key to .env file:"
echo "     ${YELLOW}nano .env${NC}"
echo
echo "  2. Activate the environment:"
echo "     ${YELLOW}conda activate rustagent${NC}"
echo
echo "  3. Run the translator:"
echo "     ${YELLOW}python c_to_rust_agent.py example.c${NC}"
echo