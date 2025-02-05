# AI Model List

A simple, maintainable list of AI model identifiers organized by capability type. This repository contains up-to-date model IDs for various AI providers including OpenAI, Anthropic, Google, and more.


## Model Categories

- `MEMO_MODELS`: Models used for memo processing and generation
- `CHAT_MODELS`: Models used for chat/conversation applications
- `GRAPH_MODELS`: Models used for graph generation

## Installation Methods

### Direct inclusion in your project

If you prefer not to use npm, you can copy `models.js` directly into your project:

1. Copy `models.js` to your project's source directory
2. Import directly from the file location:

```javascript
import { CHAT_MODELS } from './path/to/models';
```

### Git Submodule

You can also include this repository as a Git submodule:

```bash
git submodule add https://github.com/yourorg/ai-model-list.git
git submodule update --init --recursive
```