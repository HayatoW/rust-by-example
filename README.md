# agent-instructions-ja

AI Agent を日本語で使うためのよくある初期設定

## 想定 Agent

- Cursor
- Devin
- GitHub Copilot
- Windsurf Cascade

## 各ファイルの説明とリファレンス

```text
.
├── .cursor/
│   └── BUGBOT.md
├── .github/
│   └── copilot-instructions.md
├── .cursorrules
├── AGENTS.md
├── README.md
└── REVIEW.md
```

### .cursor/BUGBOT.md

https://cursor.com/docs/bugbot#rules

### .github/copilot-instructions.md

GitHub Copilot Coding Agent に日本語で PR を作成してもらうために念のため設定しています。

https://docs.github.com/en/copilot/reference/custom-instructions-support

### .cursorrules

コミットメッセージを日本語で生成するために設定しています。

https://forum.cursor.com/t/how-to-change-generate-commit-message-language/39670

### AGENTS.md

https://agents.md

### REVIEW.md

https://docs.devin.ai/work-with-devin/devin-review#review-md
