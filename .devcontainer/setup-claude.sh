#!/bin/bash

# Claude Code setup script for Codespaces
echo "Setting up Claude Code in Codespaces..."

# Install Claude Code
curl -fsSL https://claude.ai/install.sh | sh

# Add Claude Code to PATH for the current session
export PATH="$HOME/.local/bin:$PATH"

# Add to shell profile for future sessions
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc

# Verify installation
if command -v claude-code &> /dev/null; then
    echo "✅ Claude Code installed successfully"
    claude-code --version
else
    echo "❌ Claude Code installation failed"
fi

echo "🔧 To use Claude Code, you'll need to authenticate with your API key:"
echo "   claude-code auth"
echo ""
echo "📖 For more information, visit: https://docs.anthropic.com/en/docs/claude-code"