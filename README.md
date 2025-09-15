# TodoCLI - Polyglot Todo Manager

> **A powerful todo manager**: Write in Ruby, Deploy in Rust. Get 50x faster startup with SQLite storage.

TodoCLI demonstrates the **PTD (Polyglot Transpilation Development)** paradigm - develop in expressive Ruby, deploy optimized Rust binaries.

## 🚀 Quick Start

```bash
# Clone and setup
git clone https://github.com/ai-ptd-dev/ptd-ruby-cli.git -b todo-list-example
cd ptd-ruby-cli

# Install Ruby dependencies
bundle install

# Add your first todo
./bin/todocli-ruby add "Buy groceries" --priority high

# List todos
./bin/todocli-ruby list

# Complete a todo
./bin/todocli-ruby complete 1

# Compile to Rust (production)
./bin/compile

# Run Rust version (50x faster!)
./bin/todocli-rust list
```

## 📊 Performance Gains

| Metric | Ruby | Rust | Improvement |
|--------|------|------|-------------|
| **Startup Time** | 258ms | 5ms | **51.6x faster** |
| **Memory Usage** | 48MB | 2.8MB | **94% less** |
| **Benchmarks** | 91ms | 40ms | **2.3x faster** |
| **Binary Size** | 40MB+ deps | 1.1MB standalone | **97% smaller** |

## 🎯 What is PTD?

**Polyglot Transpilation Development** is a new programming paradigm where you:
1. **Develop** in high-level languages (Ruby, Python)
2. **Transpile** to system languages (Rust, Go)
3. **Deploy** optimized native binaries

[Learn more about PTD →](docs/base/ptd-paradigm.md)

## 📁 Project Structure

```
ptd-ruby-cli/
├── src/
│   ├── cli.rb              # Ruby entry point
│   ├── cli.rs              # Rust entry point (transpiled)
│   ├── commands/
│   │   ├── add.rb          # Ruby todo commands
│   │   ├── add.rs          # Rust todo commands (side-by-side!)
│   │   ├── list.rb/list.rs # List todos
│   │   ├── complete.rb/rs  # Complete todos
│   │   ├── delete.rb/rs    # Delete todos
│   │   └── version.rb/rs   # Version info
│   └── utils/
│       ├── database.rb     # Ruby SQLite utility
│       ├── database.rs     # Rust SQLite utility
│       └── ...
├── spec/
│   ├── commands/
│   │   ├── add_spec.rb     # Ruby tests
│   │   ├── list_spec.rb    # Rust tests (side-by-side!)
│   │   └── ...
├── bin/
│   ├── todocli-ruby       # Ruby runner
│   ├── todocli-rust       # Rust runner
│   ├── compile           # Build Rust binary
│   ├── test             # Run Rust tests
│   ├── rspec            # Run Ruby tests
│   └── lint             # Lint both languages
└── tmp/
    └── todocli.db        # SQLite database
```

## 🛠 Features

### Commands Included
- **add** - Add new todos with priority levels
- **list** - List todos with filtering options
- **complete** - Mark todos as completed
- **delete** - Remove todos permanently
- **version** - Version info (text/JSON)

### Utilities
- **Database** - SQLite storage for todos with full CRUD operations
- **Logger** - Colored output, progress bars, timing

### Developer Tools
- `./bin/compile` - Build optimized Rust binary
- `./bin/test` - Run Rust test suite
- `./bin/rspec` - Run Ruby test suite
- `./bin/lint` - Auto-fix code style issues

## 💻 Development Workflow

### 1. Create Ruby Command
```ruby
# src/commands/mycommand.rb
require_relative '../utils/database'

module TodoCli
  module Commands
    class MyCommand
      def execute
        db = Utils::Database.new
        puts "Hello from Ruby with database!"
        db.close
      end
    end
  end
end
```

### 2. Write Tests
```ruby
# spec/commands/mycommand_spec.rb
RSpec.describe TodoCli::Commands::MyCommand do
  it 'works' do
    expect { described_class.new.execute }
      .to output(/Hello/).to_stdout
  end
end
```

### 3. Transpile to Rust
```rust
// src/commands/mycommand.rs
use crate::utils::database::Database;
use anyhow::Result;

pub struct MyCommand;

impl MyCommand {
    pub fn execute(&self) -> Result<()> {
        let _db = Database::new()?;
        println!("Hello from Rust with database!");
        Ok(())
    }
}
```

### 4. Compile & Deploy
```bash
./bin/compile
./bin/basiccli-rust mycommand  # Instant execution!
```

## 📈 Real-World Impact

For a CLI tool run 100 times daily:
- **Ruby**: 25.8 seconds total runtime
- **Rust**: 0.5 seconds total runtime
- **Time saved**: 25.3 seconds/day (98% reduction)

In scripts processing 1000 files:
- **Ruby**: 4.3 minutes
- **Rust**: 5 seconds
- **Time saved**: 4.2 minutes (98% reduction)

## 🎓 Documentation

- [**Getting Started**](docs/guides/getting-started.md) - Setup and first steps
- [**PTD Paradigm**](docs/base/ptd-paradigm.md) - Understanding the methodology
- [**Performance Analysis**](docs/base/performance.md) - Detailed benchmarks
- [**Examples**](docs/guides/) - More guides and patterns

## 🔧 Use This Boilerplate

1. **Fork this repository**
2. **Rename** TodoCli to your project name
3. **Add commands** following the pattern
4. **Write tests** for both Ruby and Rust
5. **Deploy** the Rust binary

### Customization Example

```bash
# Fork and customize
git clone https://github.com/ai-ptd-dev/ptd-ruby-cli.git -b todo-list-example
cd ptd-ruby-cli

# Add your command
vim src/commands/deploy.rb
vim src/commands/deploy.rs

# Test both versions
./bin/rspec
./bin/test

# Ship it!
./bin/compile
cp target/release/todocli-rust /usr/local/bin/todocli
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Ensure both Ruby and Rust tests pass
4. Maintain functional parity between languages
5. Submit a pull request

## 📄 License

MIT License - Use freely in your projects

## 🌟 Why TodoCLI?

- **Best of Both Worlds**: Ruby's expressiveness, Rust's performance
- **Side-by-Side Code**: See Ruby and Rust implementations together
- **Production Ready**: Full test suites, linting, documentation
- **Real Performance**: Not theoretical - actual 50x startup improvement
- **Developer Friendly**: Helper scripts for common tasks

## 🚦 Status

- ✅ Ruby implementation complete
- ✅ Rust transpilation complete  
- ✅ Test suites passing
- ✅ Documentation complete
- ✅ Performance validated

## 🔗 Links

- [PTD Methodology](https://github.com/ai-ptd-dev)
- [Performance Report](docs/base/performance.md)
- [Getting Started Guide](docs/guides/getting-started.md)

---

**Ready to build fast CLIs?** Fork TodoCLI and experience the PTD paradigm! 🚀