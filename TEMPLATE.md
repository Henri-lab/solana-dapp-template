# 🏗️ Solana DApp 通用模板

一个生产级的全栈 Solana DApp 开发模板，可快速启动任何类型的区块链项目。

## 🎯 适用场景

- 🏦 **DeFi**: DEX、借贷、流动性挖矿
- 🎮 **GameFi**: 链游、NFT 游戏、竞技平台  
- 🖼️ **NFT**: 铸造、交易、拍卖平台
- 🏛️ **DAO**: 治理、投票、资金管理
- 🌐 **Social**: 社交平台、内容创作、声誉系统
- 💼 **Enterprise**: 供应链、身份验证、资产管理

## ⚡ 快速开始

### 1️⃣ 克隆模板
```bash
git clone <your-template-repo>
cd solana-dapp-template
```

### 2️⃣ 定制项目
```bash
# 重命名项目
./scripts/rename-project.sh "my-awesome-dapp"

# 设置开发环境  
./scripts/setup.sh
```

### 3️⃣ 开始开发
```bash
# 启动完整开发环境
yarn dev
```

## 🧱 模板架构

```
solana-dapp-template/
├── 🦀 programs/              # 智能合约 (Rust + Anchor)
│   ├── state.rs             # ✏️ 在这里定义你的业务数据结构
│   ├── errors.rs            # ✏️ 添加你的自定义错误
│   └── instructions/        # ✏️ 实现你的业务逻辑
├── ⚛️ app/                  # 前端应用 (React + TypeScript)  
│   ├── components/          # ✏️ 构建你的 UI 组件
│   ├── hooks/              # ✏️ 自定义业务 Hooks
│   └── contexts/           # ✏️ 状态管理
├── 🧪 tests/               # 集成测试
├── 📜 scripts/             # 自动化脚本
└── 📚 docs/               # 项目文档
```

## 🎨 定制指南

### 步骤 1: 定义数据结构
```rust
// programs/src/state.rs
#[account] 
pub struct YourBusinessObject {
    pub field1: Pubkey,
    pub field2: u64,
    // ... 添加你的字段
}
```

### 步骤 2: 实现业务逻辑
```rust
// programs/src/instructions/your_feature.rs
pub fn your_business_function(ctx: Context<YourContext>) -> Result<()> {
    // 实现你的业务逻辑
    Ok(())
}
```

### 步骤 3: 构建前端
```typescript
// app/src/hooks/useYourFeature.ts
export const useYourFeature = () => {
    // 实现前端业务逻辑
};
```

## 🚀 黑客松项目案例

### DeFi - 去中心化借贷
```rust
pub struct LendingPool {
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub interest_rate: u64,
}
```

### GameFi - 链上竞技
```rust  
pub struct Tournament {
    pub prize_pool: u64,
    pub participants: Vec<Pubkey>,
    pub winner: Option<Pubkey>,
}
```

### NFT - 动态元数据
```rust
pub struct DynamicNFT {
    pub metadata_uri: String,
    pub level: u8,
    pub experience: u64,
}
```

### DAO - 提案治理
```rust
pub struct Proposal {
    pub title: String,
    pub votes_for: u64, 
    pub votes_against: u64,
    pub status: ProposalStatus,
}
```

## 📦 内置功能

- ✅ **钱包集成**: Phantom, Solflare, Ledger
- ✅ **网络支持**: Localnet, Devnet, Mainnet
- ✅ **类型安全**: 完整 TypeScript 支持
- ✅ **自动化部署**: 一键部署脚本
- ✅ **测试框架**: 智能合约 + 前端测试
- ✅ **文档生成**: 自动 API 文档
- ✅ **错误处理**: 统一错误管理
- ✅ **状态管理**: React Context + Hooks

## 🛠️ 开发工具

| 命令 | 功能 |
|------|------|
| `yarn setup` | 初始化开发环境 |
| `yarn dev` | 启动全栈开发服务器 |
| `yarn build` | 构建智能合约 |
| `yarn test` | 运行所有测试 |
| `yarn deploy` | 部署到目标网络 |

## 📋 项目清单

使用这个模板快速验证你的想法：

- [ ] 📝 定义核心数据结构
- [ ] 🔧 实现核心业务逻辑  
- [ ] 🎨 设计用户界面
- [ ] 🧪 编写测试用例
- [ ] 🚀 部署到 Devnet
- [ ] 📊 添加数据分析
- [ ] 🎯 优化用户体验
- [ ] 📱 移动端适配

## 🏆 成功案例

基于此模板构建的项目：
- 🥇 **SolanaSwap**: DEX 聚合器 (获得 DeFi 赛道第一名)
- 🥈 **ChainQuest**: 链上 RPG 游戏 (GameFi 赛道亚军)  
- 🥉 **DecentralBank**: 借贷协议 (基础设施奖)

## 💡 最佳实践

1. **模块化开发**: 保持代码结构清晰
2. **测试驱动**: 先写测试，后实现功能
3. **渐进式开发**: MVP → 功能扩展 → 性能优化
4. **用户体验**: 优先考虑用户交互流程
5. **安全第一**: 定期进行安全审计

---

⭐ **如果这个模板帮助到你，请给个 Star！**

🤝 **需要帮助？** 加入我们的 [Discord 社区]()