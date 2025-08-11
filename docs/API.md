# API 文档

## 智能合约 API

### 指令 (Instructions)

#### 1. initialize_game

创建新的游戏状态账户。

**参数**: 无

**账户**:
- `game_state` (写入): 游戏状态账户 (PDA)
- `authority` (签名者): 游戏创建者
- `system_program`: 系统程序

**示例**:
```typescript
const tx = await program.methods
  .initializeGame()
  .accounts({
    gameState: gameStatePda,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

#### 2. initialize_player

创建新的玩家账户。

**参数**: 无

**账户**:
- `player` (写入): 玩家账户 (PDA)
- `authority` (签名者): 玩家公钥
- `system_program`: 系统程序

**示例**:
```typescript
const tx = await program.methods
  .initializePlayer()
  .accounts({
    player: playerPda,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

#### 3. update_score

更新玩家分数。

**参数**:
- `new_score` (u64): 新增分数

**账户**:
- `player` (写入): 玩家账户 (PDA)
- `game_state` (写入): 游戏状态账户 (PDA)
- `authority` (签名者): 玩家公钥

**错误**:
- `InvalidOperation`: 游戏未激活
- `Unauthorized`: 权限不足

**示例**:
```typescript
const tx = await program.methods
  .updateScore(new BN(100))
  .accounts({
    player: playerPda,
    gameState: gameStatePda,
    authority: wallet.publicKey,
  })
  .rpc();
```

#### 4. deactivate_game

停用游戏。

**参数**: 无

**账户**:
- `game_state` (写入): 游戏状态账户 (PDA)
- `authority` (签名者): 游戏创建者

**错误**:
- `Unauthorized`: 权限不足

**示例**:
```typescript
const tx = await program.methods
  .deactivateGame()
  .accounts({
    gameState: gameStatePda,
    authority: wallet.publicKey,
  })
  .rpc();
```

### 账户结构

#### GameState
```rust
pub struct GameState {
    pub authority: Pubkey,    // 32 bytes
    pub players: u64,         // 8 bytes
    pub total_score: u64,     // 8 bytes
    pub is_active: bool,      // 1 byte
    pub created_at: i64,      // 8 bytes
}
// Total: 57 bytes + 8 bytes discriminator = 65 bytes
```

#### Player
```rust
pub struct Player {
    pub authority: Pubkey,    // 32 bytes
    pub score: u64,          // 8 bytes
    pub games_played: u64,   // 8 bytes
    pub joined_at: i64,      // 8 bytes
}
// Total: 56 bytes + 8 bytes discriminator = 64 bytes
```

### PDA 计算

#### 游戏状态 PDA
```typescript
const [gameStatePda, bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("game"), authority.toBuffer()],
  programId
);
```

#### 玩家 PDA
```typescript
const [playerPda, bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("player"), authority.toBuffer()],
  programId
);
```

## 前端 API

### Context API

#### SolanaProvider

提供 Solana 连接和钱包管理。

**Props**:
- `children`: ReactNode

**Context 值**:
```typescript
interface SolanaContextType {
  network: Network;
  setNetwork: (network: Network) => void;
}
```

**使用**:
```typescript
const { network, setNetwork } = useSolana();
```

### 自定义 Hooks

#### useProgram

获取 Anchor 程序实例。

**返回值**:
```typescript
interface ProgramReturn {
  program: Program<Anchor20250808> | null;
  programId: PublicKey;
}
```

**使用**:
```typescript
const { program, programId } = useProgram();
```

#### useGameState

管理游戏状态。

**参数**:
- `authority`: PublicKey - 游戏创建者公钥

**返回值**:
```typescript
interface GameStateReturn {
  gameState: GameState | null;
  loading: boolean;
  error: string | null;
  initializeGame: () => Promise<string>;
  deactivateGame: () => Promise<string>;
  refresh: () => Promise<void>;
}
```

**使用**:
```typescript
const { gameState, initializeGame, deactivateGame } = useGameState(authority);
```

#### usePlayer

管理玩家数据。

**参数**:
- `authority`: PublicKey - 玩家公钥

**返回值**:
```typescript
interface PlayerReturn {
  player: Player | null;
  loading: boolean;
  error: string | null;
  initializePlayer: () => Promise<string>;
  updateScore: (score: number) => Promise<string>;
  refresh: () => Promise<void>;
}
```

**使用**:
```typescript
const { player, initializePlayer, updateScore } = usePlayer(authority);
```

### 工具函数

#### getConnection

创建 Solana 连接。

**参数**:
- `network`: 'localnet' | 'devnet' | 'mainnet-beta'

**返回值**: Connection

**使用**:
```typescript
const connection = getConnection('devnet');
```

#### getGameStateAddress

计算游戏状态 PDA 地址。

**参数**:
- `authority`: PublicKey

**返回值**: [PublicKey, number]

**使用**:
```typescript
const [gameStatePda, bump] = getGameStateAddress(authority);
```

#### getPlayerAddress

计算玩家 PDA 地址。

**参数**:
- `authority`: PublicKey

**返回值**: [PublicKey, number]

**使用**:
```typescript
const [playerPda, bump] = getPlayerAddress(authority);
```

## 错误处理

### 智能合约错误

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid account provided")]
    InvalidAccount = 6000,
    
    #[msg("Insufficient funds")]
    InsufficientFunds = 6001,
    
    #[msg("Unauthorized access")]
    Unauthorized = 6002,
    
    #[msg("Invalid operation")]
    InvalidOperation = 6003,
}
```

### 前端错误处理

```typescript
try {
  const tx = await program.methods.updateScore(new BN(100)).rpc();
  console.log('Transaction successful:', tx);
} catch (error) {
  if (error.code === 6002) {
    console.error('Unauthorized access');
  } else {
    console.error('Transaction failed:', error);
  }
}
```

## 事件监听

### 监听账户变化

```typescript
// 监听游戏状态变化
const subscription = connection.onAccountChange(
  gameStatePda,
  (accountInfo) => {
    const gameState = program.account.gameState.coder.accounts.decode(
      'GameState',
      accountInfo.data
    );
    console.log('Game state updated:', gameState);
  }
);
```

### 监听交易确认

```typescript
const tx = await program.methods.updateScore(new BN(100)).rpc();

// 等待交易确认
const confirmation = await connection.confirmTransaction(tx);
console.log('Transaction confirmed:', confirmation);
```