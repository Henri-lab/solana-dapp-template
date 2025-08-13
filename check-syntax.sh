#!/bin/bash

# 简单的语法检查脚本
echo "🔍 检查 token-economics 程序语法..."

# 切换到程序目录
cd programs/token-economics

# 检查基本语法
echo "检查基本 Rust 语法..."
if rustc --edition=2021 --crate-type lib src/lib.rs --extern anchor_lang --extern anchor_spl 2>&1; then
    echo "✅ 基本语法检查通过"
else
    echo "❌ 发现语法错误"
fi

echo "🔍 检查完成"