# Solana Game DApp

一个全栈 Solana 游戏 DApp，使用 Rust 智能合约和 React 前端。

## 🏗️ 项目架构

```
anchor-20250808/
├── programs/           # Rust 智能合约
│   └── anchor-20250808/
│       └── src/
│           ├── lib.rs           # 主合约入口
│           ├── state.rs         # 数据结构定义
│           ├── errors.rs        # 错误定义
│           └── instructions/    # 指令模块
├── app/               # React 前端应用
│   ├── src/
│   │   ├── components/     # React 组件
│   │   ├── contexts/       # React Context
│   │   ├── hooks/          # 自定义 Hooks
│   │   ├── types/          # TypeScript 类型
│   │   └── utils/          # 工具函数
├── tests/             # 集成测试
├── scripts/           # 自动化脚本
└── docs/              # 项目文档
```

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Solana CLI 1.18+
- Anchor CLI 0.31+

### 安装依赖

```bash
# 运行自动化设置脚本
./scripts/setup.sh

# 或手动安装
yarn install           # 安装根目录依赖
yarn install:frontend  # 安装前端依赖
```

### 开发流程

1. **启动本地 Solana 验证器**
   ```bash
   yarn dev:validator
   ```

2. **构建和部署智能合约**
   ```bash
   anchor build
   anchor deploy
   ```

3. **启动前端开发服务器**
   ```bash
   yarn dev:frontend
   ```

4. **或者一键启动全栈开发**
   ```bash
   yarn dev
   ```

## 📋 可用脚本命令

### 🏗️ 项目设置
| 脚本 | 描述 |
|------|------|
| `yarn setup` | 完整项目初始化（运行setup.sh脚本） |
| `yarn setup:quick` | 快速安装依赖和构建 |
| `yarn clean` | 清理所有构建文件和依赖 |

### 🔨 构建相关
| 脚本 | 描述 |
|------|------|
| `yarn build` | 构建 Anchor 程序 |
| `yarn build:all` | 构建程序和前端 |
| `yarn build:frontend` | 仅构建前端 |

### 🧪 测试相关
| 脚本 | 描述 |
|------|------|
| `yarn test` | 运行 Anchor 集成测试 |
| `yarn test:unit` | 运行 Rust 单元测试 |

### 🚀 部署相关
| 脚本 | 描述 |
|------|------|
| `yarn deploy` | 部署到配置的集群 |
| `yarn deploy:local` | 明确部署到本地网络 |

### 🔍 代码质量
| 脚本 | 描述 |
|------|------|
| `yarn lint` | 根目录代码检查 |
| `yarn lint:fix` | 根目录代码修复 |
| `yarn lint:frontend` | 前端代码检查 |
| `yarn lint:frontend:fix` | 前端代码修复 |

### ⚡ 开发服务器
| 脚本 | 描述 |
|------|------|
| `yarn dev` | 同时启动验证器和前端（推荐） |
| `yarn start` | dev 的别名 |
| `yarn dev:validator` | 仅启动本地验证器 |
| `yarn dev:frontend` | 仅启动前端开发服务器 |
| `yarn start:prod` | 生产预览模式 |

### 🔧 Solana 工具
| 脚本 | 描述 |
|------|------|
| `yarn check:config` | 查看 Solana 配置 |
| `yarn check:validator` | 检查验证器版本 |
| `yarn check:balance` | 查看钱包余额 |
| `yarn fund:wallet` | 空投 2 SOL 到钱包 |
| `yarn logs` | 查看 Solana 日志 |
| `yarn accounts` | 查看游戏状态账户 |
| `yarn idl:generate` | 生成 IDL 类型文件 |

### 🔄 完整重启
| 脚本 | 描述 |
|------|------|
| `yarn full:restart` | 清理→设置→部署→启动开发环境 |

### 📝 常用启动流程

**首次使用：**
```bash
yarn setup
yarn deploy
yarn dev
```

**日常开发：**
```bash
yarn dev
```

**完整重置：**
```bash
yarn full:restart
```

## 🎮 功能特性

### 智能合约功能
- ✅ 初始化游戏状态
- ✅ 创建玩家账户
- ✅ 更新玩家分数
- ✅ 游戏状态管理
- ✅ 权限验证

### 前端功能
- ✅ 钱包连接 (Phantom, Solflare)
- ✅ 网络切换 (Localnet, Devnet, Mainnet)
- ✅ 智能合约交互
- ✅ 实时状态显示
- ✅ 响应式设计

## 🔧 配置

### 环境变量

复制 `.env.example` 到 `.env` 并配置：

```bash
cp .env.example .env
```

主要配置项：
- `PROGRAM_ID`: 智能合约地址
- `SOLANA_NETWORK`: 网络选择
- `ANCHOR_WALLET`: 钱包路径

### 网络配置

- **Localnet**: `http://localhost:8899`
- **Devnet**: `https://api.devnet.solana.com`
- **Mainnet**: `https://api.mainnet-beta.solana.com`

## 🧪 测试

```bash
# 运行所有测试
yarn test

# 运行特定测试
anchor test tests/anchor-20250808.ts
```

## 📚 学习资源

### Rust & Anchor
- [Anchor 官方文档](https://docs.anchor-lang.com/)
- [Solana 程序开发指南](https://docs.solana.com/developing/programming-model/overview)
- [Rust 程序设计语言](https://doc.rust-lang.org/book/)

### 前端开发
- [Solana Web3.js](https://docs.solana.com/developing/clients/javascript-reference)
- [Wallet Adapter](https://github.com/solana-labs/wallet-adapter)
- [React 官方文档](https://react.dev/)

## 🛠️ 开发指南

### 添加新功能

1. **智能合约**：
   - 在 `programs/anchor-20250808/src/instructions/` 添加新指令
   - 更新 `lib.rs` 导出新函数
   - 在 `state.rs` 定义数据结构

2. **前端**：
   - 在 `app/src/components/` 添加新组件
   - 使用 `useConnection` 和 `useWallet` 钩子
   - 在 `utils/solana.ts` 添加工具函数

### 代码规范

```bash
# Rust 代码格式化
cargo fmt

# TypeScript 代码检查
cd app && npm run lint
```

## 🚨 常见问题

### 1. 程序部署失败
```bash
# 检查余额
solana balance

# 空投测试代币
solana airdrop 2
```

### 2. 前端连接失败
- 确保本地验证器正在运行
- 检查程序 ID 是否正确
- 验证网络配置

### 3. 测试失败
- 确保程序已部署
- 检查测试账户余额
- 重置验证器状态


## 📄 许可证

ISC License