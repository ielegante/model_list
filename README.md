# AI Model List Generator

A simple repository containing AI model identifiers and a generator script. The repository uses a single source of truth (`models.json`) and generates various formats for different use cases.

## Structure

```
.
├── models.json         # Source of truth - all model definitions
├── src/
│   └── main.rs        # Generator script
├── generated/
│   ├── models.csv     # Generated CSV format
│   └── models.md      # Generated Markdown documentation
└── Cargo.toml         # Rust project configuration
```

## Usage

### Directly Using models.json

The `models.json` file can be directly imported into your projects. It contains model IDs organized by capability (memo, chat, graph):

```javascript
import models from './models.json';

// Access model IDs
const chatModelId = models.models.chat.GPT_4O;  // "gpt-4o-2024-11-20"
```

### Generating Other Formats

To generate CSV and Markdown formats:

1. Install Rust if you haven't already: [https://rustup.rs/](https://rustup.rs/)

2. Run the generator:
```bash
cargo run
```

This will create the following files in the `generated` directory:
- `models.csv`: Spreadsheet-friendly format
- `models.md`: Human-readable documentation

## Updating Models

To add or update models:

1. Edit `models.json`
2. Run the generator
3. Commit changes

## Format Details

### models.json
```json
{
  "models": {
    "memo": {
      "MODEL_NAME": "model-id",
      ...
    },
    "chat": {
      ...
    },
    "graph": {
      ...
    }
  }
}
```

### Generated CSV
```csv
category,model_name,model_id
memo,GPT_4O,gpt-4o-2024-11-20
...
```

### Generated Markdown
Organized by capability type with tables for each category.