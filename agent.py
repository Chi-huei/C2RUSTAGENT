import os
import sys
import subprocess
import tempfile
from pathlib import Path
from typing import TypedDict, Annotated, Literal
from dotenv import load_dotenv

from langchain_anthropic import ChatAnthropic
from langchain_core.messages import HumanMessage, SystemMessage, AIMessage
from langchain_core.prompts import ChatPromptTemplate
from langgraph.graph import StateGraph, END
from tqdm import tqdm
load_dotenv()


# State

class AgentState(TypedDict):
    c_code: str
    rust_code: str
    compile_output: str
    compile_success: bool
    attempt: int
    max_attempts: int
    error_history: list[str]
    final_result: str
    preserve_main: bool  # If False, strip main() from final output


# LLM Setup

def get_llm():
    """Initialize the Claude model via LangChain."""
    return ChatAnthropic(
        model="claude-sonnet-4-20250514",
        temperature=0.2,
        max_tokens=8192,
    )


# Translation Prompt

TRANSLATION_SYSTEM_PROMPT = """You are an expert systems programmer fluent in both C and Rust.
Your task is to translate C code to idiomatic, safe Rust code.

Guidelines:
1. Preserve the logic and functionality exactly
2. Use safe Rust constructs wherever possible
3. Replace C pointers with appropriate Rust types (references, Box, Vec, etc.)
4. Handle memory management using Rust's ownership system
5. Use Result/Option for error handling instead of error codes
6. Convert C-style strings to Rust String or &str
7. Use Rust standard library equivalents for C stdlib functions
8. Add necessary 'use' statements for imports
9. Ensure the code compiles without warnings
10. Include a main() function if the C code has one

Output ONLY the Rust code, no explanations or markdown code blocks."""

FIX_COMPILATION_PROMPT = """You are an expert Rust programmer. The following Rust code failed to compile.

Original C code:
```c
{c_code}
```

Current Rust translation:
```rust
{rust_code}
```

Compilation error:
```
{error}
```

Previous errors encountered:
{error_history}

Fix the Rust code to resolve the compilation error while maintaining the original C program's functionality.
Output ONLY the corrected Rust code, no explanations or markdown code blocks."""


# Agent Nodes

def translate_node(state: AgentState) -> AgentState:
    """Translate C code to Rust using the LLM."""
    llm = get_llm()
    
    if state["attempt"] == 0:
        # Initial translation
        messages = [
            SystemMessage(content=TRANSLATION_SYSTEM_PROMPT),
            HumanMessage(content=f"Translate this C code to Rust:\n\n{state['c_code']}")
        ]
    else:
        # Fix compilation errors
        prompt = FIX_COMPILATION_PROMPT.format(
            c_code=state["c_code"],
            rust_code=state["rust_code"],
            error=state["compile_output"],
            error_history="\n".join(state["error_history"][-3:]) if state["error_history"] else "None"
        )
        messages = [
            SystemMessage(content="You are an expert Rust programmer. Fix compilation errors."),
            HumanMessage(content=prompt)
        ]
    
    response = llm.invoke(messages)
    rust_code = response.content.strip()
    
    # Clean up any markdown formatting if present
    if rust_code.startswith("```rust"):
        rust_code = rust_code[7:]
    if rust_code.startswith("```"):
        rust_code = rust_code[3:]
    if rust_code.endswith("```"):
        rust_code = rust_code[:-3]
    rust_code = rust_code.strip()
    
    return {**state, "rust_code": rust_code}


def compile_node(state: AgentState) -> AgentState:
    """Compile the Rust code and capture any errors."""
    with tempfile.TemporaryDirectory() as tmpdir:
        rust_file = Path(tmpdir) / "translated.rs"
        output_file = Path(tmpdir) / "translated"
        
        # Write Rust code to file
        rust_file.write_text(state["rust_code"])
        
        # Attempt compilation
        result = subprocess.run(
            ["rustc", str(rust_file), "-o", str(output_file)],
            capture_output=True,
            text=True,
            timeout=60
        )
        
        if result.returncode == 0:
            # Try running the compiled program to verify it works
            try:
                run_result = subprocess.run(
                    [str(output_file)],
                    capture_output=True,
                    text=True,
                    timeout=10
                )
                output = f"Compilation successful!\nProgram output:\n{run_result.stdout}"
                if run_result.stderr:
                    output += f"\nStderr: {run_result.stderr}"
                return {
                    **state,
                    "compile_output": output,
                    "compile_success": True,
                }
            except subprocess.TimeoutExpired:
                return {
                    **state,
                    "compile_output": "Compilation successful! (Program timed out during test run)",
                    "compile_success": True,
                }
            except Exception as e:
                return {
                    **state,
                    "compile_output": f"Compilation successful! (Could not run: {e})",
                    "compile_success": True,
                }
        else:
            error_msg = result.stderr or result.stdout or "Unknown compilation error"
            error_history = state["error_history"] + [error_msg]
            return {
                **state,
                "compile_output": error_msg,
                "compile_success": False,
                "attempt": state["attempt"] + 1,
                "error_history": error_history,
            }


def strip_main_function(rust_code: str) -> str:
    """
    Remove the main() function from Rust code.
    Handles both 'fn main()' and 'pub fn main()' variants.
    """
    lines = rust_code.split('\n')
    result_lines = []
    brace_count = 0
    inside_main = False
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Detect start of main function
        if not inside_main and ('fn main()' in stripped or 'fn main ()' in stripped):
            inside_main = True
            # Count opening brace on same line or find it
            brace_count = line.count('{') - line.count('}')
            
            # If no brace on this line, look for it on next lines
            if brace_count == 0 and '{' not in line:
                i += 1
                while i < len(lines) and '{' not in lines[i]:
                    i += 1
                if i < len(lines):
                    brace_count = lines[i].count('{') - lines[i].count('}')
            
            # If main body is complete on same line
            if brace_count == 0 and '{' in line:
                inside_main = False
            i += 1
            continue
        
        if inside_main:
            brace_count += line.count('{') - line.count('}')
            if brace_count <= 0:
                inside_main = False
            i += 1
            continue
        
        result_lines.append(line)
        i += 1
    
    # Clean up extra blank lines at the end
    while result_lines and result_lines[-1].strip() == '':
        result_lines.pop()
    
    return '\n'.join(result_lines)


def finalize_node(state: AgentState) -> AgentState:
    """Finalize the result based on compilation success."""
    # Prepare the final Rust code (strip main if requested)
    final_rust_code = state["rust_code"]
    if state["compile_success"] and not state["preserve_main"]:
        final_rust_code = strip_main_function(state["rust_code"])
    
    main_note = "" if state["preserve_main"] else "\n(main() function stripped from output)\n"
    
    if state["compile_success"]:
        result = f"""
╔══════════════════════════════════════════════════════════════╗
║           TRANSLATION SUCCESSFUL                             ║
╚══════════════════════════════════════════════════════════════╝

Attempts: {state['attempt'] + 1}
{main_note}
{state['compile_output']}

═══════════════════════ RUST CODE ═══════════════════════

{final_rust_code}

══════════════════════════════════════════════════════════
"""
    else:
        result = f"""
╔══════════════════════════════════════════════════════════════╗
║           TRANSLATION FAILED                                 ║
╚══════════════════════════════════════════════════════════════╝

Max attempts ({state['max_attempts']}) reached.

Last compilation error:
{state['compile_output']}

═══════════════════ LAST RUST CODE ═══════════════════

{state['rust_code']}

══════════════════════════════════════════════════════════
"""
    return {**state, "rust_code": final_rust_code, "final_result": result}


# =============================================================================
# Conditional Edge
# =============================================================================

def should_retry(state: AgentState) -> Literal["translate", "finalize"]:
    """Determine if we should retry translation or finalize."""
    if state["compile_success"]:
        return "finalize"
    if state["attempt"] >= state["max_attempts"]:
        return "finalize"
    return "translate"


# Agent Graph
def build_agent():
    """Build the LangGraph agent workflow."""
    workflow = StateGraph(AgentState)
    
    # Add nodes
    workflow.add_node("translate", translate_node)
    workflow.add_node("compile", compile_node)
    workflow.add_node("finalize", finalize_node)
    
    # Set entry point
    workflow.set_entry_point("translate")
    
    # Add edges
    workflow.add_edge("translate", "compile")
    workflow.add_conditional_edges(
        "compile",
        should_retry,
        {
            "translate": "translate",
            "finalize": "finalize",
        }
    )
    workflow.add_edge("finalize", END)
    
    return workflow.compile()


# Main Entry Point

def translate_c_to_rust(c_code: str, max_attempts: int = 5, preserve_main: bool = True) -> dict:
    agent = build_agent()
    
    initial_state: AgentState = {
        "c_code": c_code,
        "rust_code": "",
        "compile_output": "",
        "compile_success": False,
        "attempt": 0,
        "max_attempts": max_attempts,
        "error_history": [],
        "final_result": "",
        "preserve_main": preserve_main,
    }
    
    # Run the agent
    final_state = agent.invoke(initial_state)
    return final_state


def check_prerequisites():
    """Check for rustc and API key. Exit if not found."""
    try:
        subprocess.run(["rustc", "--version"], capture_output=True, check=True)
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("Error: 'rustc' not found. Please install Rust: https://rustup.rs")
        sys.exit(1)
    
    if not os.getenv("ANTHROPIC_API_KEY"):
        print("Error: ANTHROPIC_API_KEY environment variable not set")
        sys.exit(1)


def cli():
    """CLI entry point for single file translation."""
    if len(sys.argv) < 2:
        print("Usage: python c_to_rust_agent.py <file.c> [max_attempts] [--no-main]")
        print("\nOptions:")
        print("  --no-main    Strip the main() function from the output")
        print("\nExamples:")
        print("  python c_to_rust_agent.py hello.c")
        print("  python c_to_rust_agent.py complex.c 10")
        print("  python c_to_rust_agent.py leetcode.c --no-main")
        print("  python c_to_rust_agent.py leetcode.c 5 --no-main")
        print("\nBatch mode:")
        print("  python c_to_rust_agent.py --auto <indir> <outdir> [max_attempts]")
        sys.exit(1)
    
    # Check for auto mode
    if sys.argv[1] == "--auto":
        if len(sys.argv) < 4:
            print("Usage: python c_to_rust_agent.py --auto <indir> <outdir> [max_attempts]")
            sys.exit(1)
        indir = sys.argv[2]
        outdir = sys.argv[3]
        max_attempts = int(sys.argv[4]) if len(sys.argv) > 4 else 5
        return auto(indir, outdir, max_attempts)
    
    # Parse arguments for single file mode
    args = sys.argv[1:]
    preserve_main = "--no-main" not in args
    args = [a for a in args if a != "--no-main"]
    
    c_file = Path(args[0])
    max_attempts = int(args[1]) if len(args) > 1 else 5
    
    if not c_file.exists():
        print(f"Error: File '{c_file}' not found")
        sys.exit(1)
    
    if not c_file.suffix == ".c":
        print(f"Warning: File '{c_file}' does not have .c extension")
    
    check_prerequisites()
    
    print(f"Reading C code from: {c_file}")
    c_code = c_file.read_text()
    
    print(f"Starting translation (max {max_attempts} attempts)...")
    if not preserve_main:
        print("Note: main() function will be stripped from output")
    print()
    
    result = translate_c_to_rust(c_code, max_attempts, preserve_main)
    
    print(result["final_result"])
    
    # Optionally save the Rust code
    if result["compile_success"]:
        rust_file = c_file.with_suffix(".rs")
        rust_file.write_text(result["rust_code"])
        print(f"Rust code saved to: {rust_file}")
    
    return 0 if result["compile_success"] else 1


def auto(indir: str, outdir: str, max_attempts: int = 5) -> int:
    indir_path = Path(indir)
    outdir_path = Path(outdir)
    
    if not indir_path.exists():
        print(f"Error: Input directory '{indir}' not found")
        return 1
    
    if not indir_path.is_dir():
        print(f"Error: '{indir}' is not a directory")
        return 1
    
    check_prerequisites()
    
    # Find all .c files
    c_files = sorted(indir_path.glob("*.c"))
    
    if not c_files:
        print(f"No .c files found in '{indir}'")
        return 1
    
    print("═" * 70)
    print(f"  C to Rust Batch Translation")
    print("═" * 70)
    print(f"  Input directory:  {indir_path.absolute()}")
    print(f"  Output directory: {outdir_path.absolute()}")
    print(f"  Files to process: {len(c_files)}")
    print(f"  Max attempts:     {max_attempts}")
    print("═" * 70)
    print()
    
    # Create output directory
    outdir_path.mkdir(parents=True, exist_ok=True)
    
    # Track results
    results = {"success": [], "failed": []}
    
    for i, c_file in enumerate(c_files, 1):
        file_stem = c_file.stem  # filename without extension
        
        print(f"[{i}/{len(c_files)}] Processing: {c_file.name}")
        print("-" * 50)
        
        # Create subdirectory for this file
        file_outdir = outdir_path / file_stem
        file_outdir.mkdir(parents=True, exist_ok=True)
        
        # Copy original C file
        c_code = c_file.read_text()
        dest_c_file = file_outdir / c_file.name
        dest_c_file.write_text(c_code)
        
        # Translate (always strip main for batch mode)
        try:
            result = translate_c_to_rust(c_code, max_attempts, preserve_main=False)
            
            if result["compile_success"]:
                # Save Rust file
                rust_file = file_outdir / f"{file_stem}.rs"
                rust_file.write_text(result["rust_code"])
                
                print(f"  ✓ Success (attempts: {result['attempt'] + 1})")
                print(f"    → {rust_file}")
                results["success"].append(c_file.name)
            else:
                print(f"  ✗ Failed after {max_attempts} attempts")
                print(f"    Error: {result['compile_output'][:100]}...")
                results["failed"].append(c_file.name)
                
                # Save failed Rust code for debugging
                rust_file = file_outdir / f"{file_stem}.rs.failed"
                rust_file.write_text(result["rust_code"])
                
        except Exception as e:
            print(f"  ✗ Exception: {e}")
            results["failed"].append(c_file.name)
        
        print()
    
    # Print summary
    print("═" * 70)
    print("  BATCH TRANSLATION SUMMARY")
    print("═" * 70)
    print(f"  Total files:  {len(c_files)}")
    print(f"  Successful:   {len(results['success'])}")
    print(f"  Failed:       {len(results['failed'])}")
    print()
    
    if results["success"]:
        print("  ✓ Successful translations:")
        for name in results["success"]:
            print(f"      {name}")
    
    if results["failed"]:
        print("\n  ✗ Failed translations:")
        for name in results["failed"]:
            print(f"      {name}")
    
    print("═" * 70)
    
    return 0 if not results["failed"] else 1


if __name__ == "__main__":
    sys.exit(cli())