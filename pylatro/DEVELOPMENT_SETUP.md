# Pylatro Development Setup Guide

## Quick Start

You're now ready to develop and debug the pylatro project! Here's what's been set up:

### ✅ What's Working
- Python virtual environment with Python 3.14
- Maturin for building Python-Rust bindings
- Pylatro package built and installed in development mode
- VS Code debugging configurations
- Basic functionality tests passing

### 🚀 Getting Started

#### 1. Activate the Environment
```bash
cd /Users/critter/Dropbox/code/balatro-rs/pylatro
source .env/bin/activate
```

#### 2. Test Basic Functionality
```bash
python test/main.py
```

#### 3. Run Examples
```bash
python examples/simulation.py
```

#### 4. Debug in VS Code
- Open VS Code in the project root
- Go to Run and Debug (Ctrl+Shift+D)
- Select one of the pylatro debug configurations:
  - "Debug Pylatro Test" - Debug the main test file
  - "Debug Pylatro Simulation" - Debug the simulation example
  - "Debug Pylatro Gym Training" - Debug the RL training code

### 🔧 Development Workflow

#### Making Changes to Rust Code
1. Edit Rust files in `core/src/` or `pylatro/src/`
2. Rebuild the Python bindings:
   ```bash
   source .env/bin/activate
   PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop
   ```
3. Test your changes:
   ```bash
   python test/main.py
   ```

#### Making Changes to Python Code
1. Edit Python files in `pylatro/`
2. Test immediately (no rebuild needed):
   ```bash
   python test/main.py
   ```

### 🐛 Debugging

#### Python Debugging
- Use VS Code's Python debugger with the provided configurations
- Set breakpoints in Python files
- Step through code execution
- Inspect variables and call stack

#### Rust Debugging
- For Rust code debugging, use the existing Rust debug configurations
- Set breakpoints in Rust source files
- Use LLDB/GDB for low-level debugging

### 📁 Project Structure

```
pylatro/
├── .env/                    # Python virtual environment
├── src/lib.rs              # Rust bindings code
├── test/                   # Python tests
├── examples/               # Example usage
├── gym/                    # Reinforcement learning integration
├── Cargo.toml             # Rust dependencies
├── pyproject.toml         # Python package configuration
├── setup.sh               # Setup script
└── activate_env.sh        # Environment activation helper
```

### 🔍 Key Files to Know

- `test/main.py` - Basic functionality test
- `examples/simulation.py` - Game simulation example
- `gym/train.py` - RL training code
- `src/lib.rs` - Main Rust bindings

### ⚠️ Important Notes

1. **Python Version Compatibility**: You're using Python 3.14, which requires the `PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1` flag when building.

2. **Rebuild Required**: After changing Rust code, you must rebuild with:
   ```bash
   PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop
   ```

3. **Environment Activation**: Always activate the virtual environment before running Python code:
   ```bash
   source .env/bin/activate
   ```

### 🎯 Next Steps

1. **Explore the API**: Check out `examples/simulation.py` to see how to use pylatro
2. **Run Tests**: Try different test files in the `test/` directory
3. **Experiment**: Create your own Python scripts using the pylatro API
4. **Debug**: Use VS Code's debugger to step through code execution

### 🆘 Troubleshooting

- **Import Error**: Make sure the virtual environment is activated
- **Build Error**: Use the compatibility flag: `PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop`
- **Permission Error**: Make sure scripts are executable: `chmod +x setup.sh activate_env.sh`

You're all set! Happy coding! 🎉
