# 项目架构文档

## 整体架构

```
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│                     │    │                     │    │                     │
│   React Frontend    │◄──►│   Solana Blockchain │◄──►│   Anchor Program    │
│                     │    │                     │    │                     │
└─────────────────────┘    └─────────────────────┘    └─────────────────────┘
         │                           │                           │
         ▼                           ▼                           ▼
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│ Wallet Integration  │    │   RPC Connection    │    │   State Management  │
│ - Phantom           │    │   - Localnet        │    │   - GameState       │
│ - Solflare          │    │   - Devnet          │    │   - Player          │
│ - Ledger            │    │   - Mainnet         │    │   - Instructions    │
└─────────────────────┘    └─────────────────────┘    └─────────────────────┘
```

## 智能合约架构

### 程序结构
```
programs/anchor-20250808/src/
├── lib.rs                  # 程序入口点
├── state.rs               # 账户状态定义
├── errors.rs              # 自定义错误
└── instructions/          # 指令处理模块
    ├── mod.rs
    ├── initialize.rs      # 初始化指令
    └── update.rs          # 更新指令
```

### 状态设计

#### GameState 账户
```rust
pub struct GameState {
    pub authority: Pubkey,    // 游戏创建者
    pub players: u64,         // 玩家数量
    pub total_score: u64,     // 总分数
    pub is_active: bool,      // 游戏状态
    pub created_at: i64,      // 创建时间
}
```

#### Player 账户
```rust
pub struct Player {
    pub authority: Pubkey,    // 玩家公钥
    pub score: u64,          // 玩家分数
    pub games_played: u64,   // 游戏次数
    pub joined_at: i64,      // 加入时间
}
```

### PDA (Program Derived Address) 设计

```
GameState PDA: ["game", authority] → 唯一游戏状态账户
Player PDA:    ["player", authority] → 唯一玩家账户
```

## 前端架构

### 组件层次结构
```
App
├── SolanaProvider          # Solana 连接提供者
│   ├── ConnectionProvider  # RPC 连接
│   ├── WalletProvider     # 钱包连接
│   └── WalletModalProvider # 钱包模态框
├── Header                 # 头部导航
│   ├── NetworkSelector    # 网络选择
│   └── WalletButton       # 钱包连接按钮
└── GameInterface          # 游戏主界面
    ├── GameControls       # 游戏控制
    ├── PlayerStats        # 玩家统计
    └── Leaderboard        # 排行榜
```

### 状态管理

#### React Context
- `SolanaContext`: 管理网络和连接状态
- `GameContext`: 管理游戏状态
- `PlayerContext`: 管理玩家数据

#### 自定义 Hooks
- `useProgram`: 智能合约程序实例
- `useGameState`: 游戏状态管理
- `usePlayer`: 玩家数据管理

### 数据流

```
用户交互 → React组件 → Hook → Anchor程序 → 区块链状态 → 更新UI
```

## 开发工作流

### 1. 智能合约开发
```bash
1. 修改 Rust 代码
2. anchor build        # 编译程序
3. anchor test         # 运行测试
4. anchor deploy       # 部署程序
```

### 2. 前端开发
```bash
1. 修改 React 代码
2. npm run dev         # 启动开发服务器
3. 浏览器测试交互
4. npm run build       # 构建生产版本
```

### 3. 集成测试
```bash
1. solana-test-validator # 启动本地验证器
2. anchor test           # 运行智能合约测试
3. npm run test          # 运行前端测试
```

## 安全考虑

### 智能合约安全
- ✅ 账户验证 (has_one 约束)
- ✅ 权限检查 (Signer 验证)
- ✅ 数据验证 (require! 宏)
- ✅ 溢出保护 (checked arithmetic)

### 前端安全
- ✅ 输入验证
- ✅ 交易确认
- ✅ 错误处理
- ✅ 用户反馈

## 扩展指南

### 添加新指令
1. 在 `instructions/` 创建新模块
2. 定义 Context 结构体
3. 实现指令逻辑
4. 更新 `lib.rs` 导出
5. 编写测试用例

### 添加新状态
1. 在 `state.rs` 定义结构体
2. 计算账户大小 (LEN 常量)
3. 更新相关指令
4. 添加前端类型定义

### 性能优化
- 使用 zero-copy 反序列化
- 批量处理交易
- 缓存程序账户
- 预计算 PDA 地址