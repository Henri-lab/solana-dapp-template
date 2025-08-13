import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenEconomics } from "../target/types/token_economics";
import { 
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddress,
  mintTo,
  getAccount
} from "@solana/spl-token";
import { assert } from "chai";

describe("token-economics", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenEconomics as Program<TokenEconomics>;
  const authority = provider.wallet.publicKey;
  
  let stakeMint: anchor.web3.PublicKey;
  let rewardMint: anchor.web3.PublicKey;
  let userStakeTokenAccount: anchor.web3.PublicKey;
  let userRewardTokenAccount: anchor.web3.PublicKey;
  let economicsAddress: anchor.web3.PublicKey;
  let stakingPoolAddress: anchor.web3.PublicKey;
  let userStakeAddress: anchor.web3.PublicKey;

  const POOL_ID = 1;
  const REWARD_RATE = new anchor.BN(100); // 100 tokens per second
  const GOVERNANCE_FEE = 500; // 5%
  const MIN_STAKE = new anchor.BN(1000); // 1000 tokens minimum
  const MAX_STAKE = new anchor.BN(1000000); // 1M tokens maximum

  before(async () => {
    console.log("🚀 设置测试环境...");

    // Create stake token mint (用于质押的代币)
    stakeMint = await createMint(
      provider.connection,
      provider.wallet.payer,
      authority,
      null,
      9 // 9 decimals
    );

    // Create reward token mint (用于奖励的代币)
    rewardMint = await createMint(
      provider.connection,
      provider.wallet.payer,
      authority,
      null,
      9 // 9 decimals
    );

    // Get PDA addresses
    [economicsAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("economics")],
      program.programId
    );

    [stakingPoolAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), Buffer.from([POOL_ID])],
      program.programId
    );

    [userStakeAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user_stake"), authority.toBuffer(), Buffer.from([POOL_ID])],
      program.programId
    );

    // Create user token accounts
    userStakeTokenAccount = await getAssociatedTokenAddress(stakeMint, authority);
    userRewardTokenAccount = await getAssociatedTokenAddress(rewardMint, authority);

    console.log("📍 Economics Address:", economicsAddress.toString());
    console.log("📍 Pool Address:", stakingPoolAddress.toString());
    console.log("📍 User Stake Address:", userStakeAddress.toString());
  });

  describe("🏗️ 系统初始化", () => {
    it("初始化代币经济系统", async () => {
      console.log("💾 初始化经济系统...");

      const tx = await program.methods
        .initializeEconomics(
          REWARD_RATE,
          GOVERNANCE_FEE,
          MIN_STAKE,
          MAX_STAKE
        )
        .accounts({
          authority: authority,
          stakeMint: stakeMint,
          rewardMint: rewardMint,
        })
        .rpc();

      console.log("✅ 初始化交易签名:", tx);

      // 验证经济系统状态
      const economics = await program.account.tokenEconomics.fetch(economicsAddress);
      assert.equal(economics.authority.toString(), authority.toString());
      assert.equal(economics.stakeMint.toString(), stakeMint.toString());
      assert.equal(economics.rewardMint.toString(), rewardMint.toString());
      assert.equal(economics.rewardRatePerSecond.toString(), REWARD_RATE.toString());
      assert.equal(economics.governanceFeeBps, GOVERNANCE_FEE);
      assert.equal(economics.minStakeAmount.toString(), MIN_STAKE.toString());
      assert.equal(economics.maxStakeAmount.toString(), MAX_STAKE.toString());
      assert.equal(economics.totalStaked.toNumber(), 0);
      assert.equal(economics.activeStakers.toNumber(), 0);
      assert.isFalse(economics.isPaused);
      assert.isFalse(economics.emergencyMode);

      console.log("📊 经济系统初始化完成");
      console.log("  - 奖励率:", economics.rewardRatePerSecond.toString(), "代币/秒");
      console.log("  - 治理费用:", economics.governanceFeeBps / 100, "%");
      console.log("  - 最小质押:", economics.minStakeAmount.toString(), "代币");
      console.log("  - 最大质押:", economics.maxStakeAmount.toString(), "代币");
    });

    it("创建质押池", async () => {
      console.log("🏊 创建质押池...");

      const rewardMultiplier = 150; // 1.5x multiplier
      const minStakePeriod = new anchor.BN(86400); // 1 day in seconds
      const maxCapacity = new anchor.BN(10000000); // 10M tokens

      const tx = await program.methods
        .createStakingPool(
          POOL_ID,
          rewardMultiplier,
          minStakePeriod,
          maxCapacity
        )
        .accounts({
          authority: authority,
        })
        .rpc();

      console.log("✅ 创建池交易签名:", tx);

      // 验证质押池状态
      const pool = await program.account.stakingPool.fetch(stakingPoolAddress);
      assert.equal(pool.poolId, POOL_ID);
      assert.equal(pool.economics.toString(), economicsAddress.toString());
      assert.equal(pool.rewardMultiplier, rewardMultiplier);
      assert.equal(pool.minStakePeriod.toString(), minStakePeriod.toString());
      assert.equal(pool.maxCapacity.toString(), maxCapacity.toString());
      assert.equal(pool.totalStaked.toNumber(), 0);
      assert.equal(pool.activeStakers, 0);
      assert.isTrue(pool.isActive);

      console.log("🏊 质押池创建完成");
      console.log("  - 池ID:", pool.poolId);
      console.log("  - 奖励倍数:", pool.rewardMultiplier / 100, "x");
      console.log("  - 最小质押期:", pool.minStakePeriod.toNumber() / 86400, "天");
      console.log("  - 最大容量:", pool.maxCapacity.toString(), "代币");
    });
  });

  describe("🪙 代币操作", () => {
    it("铸造测试代币", async () => {
      console.log("🏭 铸造测试代币...");

      // 给用户铸造质押代币
      await mintTo(
        provider.connection,
        provider.wallet.payer,
        stakeMint,
        userStakeTokenAccount,
        authority,
        10000 * 10**9 // 10,000 tokens
      );

      // 给奖励金库铸造奖励代币
      const rewardVaultAddress = await getAssociatedTokenAddress(rewardMint, economicsAddress, true);
      await mintTo(
        provider.connection,
        provider.wallet.payer,
        rewardMint,
        rewardVaultAddress,
        authority,
        1000000 * 10**9 // 1M reward tokens
      );

      // 验证余额
      const userStakeBalance = await getAccount(provider.connection, userStakeTokenAccount);
      const rewardVaultBalance = await getAccount(provider.connection, rewardVaultAddress);

      console.log("💰 代币铸造完成");
      console.log("  - 用户质押代币余额:", Number(userStakeBalance.amount) / 10**9);
      console.log("  - 奖励金库余额:", Number(rewardVaultBalance.amount) / 10**9);

      assert.equal(Number(userStakeBalance.amount), 10000 * 10**9);
      assert.equal(Number(rewardVaultBalance.amount), 1000000 * 10**9);
    });

    it("用户质押代币", async () => {
      console.log("💎 用户开始质押...");

      const stakeAmount = new anchor.BN(5000 * 10**9); // 5000 tokens

      const tx = await program.methods
        .stakeTokens(POOL_ID, stakeAmount)
        .accounts({
          user: authority,
        })
        .rpc();

      console.log("✅ 质押交易签名:", tx);

      // 验证用户质押状态
      const userStake = await program.account.userStake.fetch(userStakeAddress);
      const pool = await program.account.stakingPool.fetch(stakingPoolAddress);
      const economics = await program.account.tokenEconomics.fetch(economicsAddress);

      assert.equal(userStake.user.toString(), authority.toString());
      assert.equal(userStake.poolId, POOL_ID);
      assert.equal(userStake.totalStaked.toString(), stakeAmount.toString());
      assert.equal(userStake.pendingRewards.toNumber(), 0);
      assert.equal(pool.totalStaked.toString(), stakeAmount.toString());
      assert.equal(pool.activeStakers, 1);
      assert.equal(economics.totalStaked.toString(), stakeAmount.toString());
      assert.equal(economics.activeStakers.toNumber(), 1);

      console.log("💎 质押完成");
      console.log("  - 质押数量:", Number(userStake.totalStaked) / 10**9, "代币");
      console.log("  - 池总质押:", Number(pool.totalStaked) / 10**9, "代币");
      console.log("  - 活跃质押者:", pool.activeStakers);
    });

    it("等待奖励累积", async () => {
      console.log("⏰ 等待奖励累积 (3秒)...");
      await new Promise(resolve => setTimeout(resolve, 3000));
      console.log("✅ 等待完成");
    });

    it("用户领取奖励", async () => {
      console.log("💸 用户领取奖励...");

      // 获取领取前的余额
      const beforeBalance = await getAccount(provider.connection, userRewardTokenAccount);
      console.log("  - 领取前余额:", Number(beforeBalance.amount) / 10**9, "奖励代币");

      const tx = await program.methods
        .claimRewards()
        .accounts({
          user: authority,
        })
        .rpc();

      console.log("✅ 领取奖励交易签名:", tx);

      // 验证奖励领取后的状态
      const userStake = await program.account.userStake.fetch(userStakeAddress);
      const economics = await program.account.tokenEconomics.fetch(economicsAddress);
      const afterBalance = await getAccount(provider.connection, userRewardTokenAccount);

      const rewardReceived = Number(afterBalance.amount) - Number(beforeBalance.amount);

      console.log("💸 奖励领取完成");
      console.log("  - 待领取奖励重置:", userStake.pendingRewards.toNumber());
      console.log("  - 实际获得奖励:", rewardReceived / 10**9, "代币");
      console.log("  - 累计奖励已领取:", Number(userStake.totalRewardsClaimed) / 10**9, "代币");
      console.log("  - 系统总奖励分发:", Number(economics.totalRewardsDistributed) / 10**9, "代币");

      assert.equal(userStake.pendingRewards.toNumber(), 0);
      assert.isTrue(rewardReceived > 0);
      assert.isTrue(economics.totalRewardsDistributed.toNumber() > 0);
    });

    it("部分解质押", async () => {
      console.log("🔓 用户部分解质押...");

      const unstakeAmount = new anchor.BN(2000 * 10**9); // 2000 tokens

      // 先等待最小质押期 (这里跳过，因为我们设置的是1天，测试中无法等待)
      // 在实际部署中，需要遵守最小质押期

      const tx = await program.methods
        .unstakeTokens(unstakeAmount)
        .accounts({
          user: authority,
        })
        .rpc();

      console.log("✅ 解质押交易签名:", tx);

      // 验证解质押后的状态
      const userStake = await program.account.userStake.fetch(userStakeAddress);
      const pool = await program.account.stakingPool.fetch(stakingPoolAddress);
      const economics = await program.account.tokenEconomics.fetch(economicsAddress);

      const expectedRemaining = new anchor.BN(3000 * 10**9); // 5000 - 2000

      console.log("🔓 解质押完成");
      console.log("  - 剩余质押:", Number(userStake.totalStaked) / 10**9, "代币");
      console.log("  - 池总质押:", Number(pool.totalStaked) / 10**9, "代币");
      console.log("  - 活跃质押者:", pool.activeStakers);

      assert.equal(userStake.totalStaked.toString(), expectedRemaining.toString());
      assert.equal(pool.totalStaked.toString(), expectedRemaining.toString());
      assert.equal(economics.totalStaked.toString(), expectedRemaining.toString());
      assert.equal(pool.activeStakers, 1); // 仍然是活跃用户
    });
  });

  describe("⚙️ 管理员功能", () => {
    it("暂停系统", async () => {
      console.log("⏸️ 暂停系统...");

      const tx = await program.methods
        .setPauseState(true)
        .accounts({
          authority: authority,
        })
        .rpc();

      console.log("✅ 暂停交易签名:", tx);

      const economics = await program.account.tokenEconomics.fetch(economicsAddress);
      assert.isTrue(economics.isPaused);

      console.log("⏸️ 系统已暂停");
    });

    it("恢复系统", async () => {
      console.log("▶️ 恢复系统...");

      const tx = await program.methods
        .setPauseState(false)
        .accounts({
          authority: authority,
        })
        .rpc();

      console.log("✅ 恢复交易签名:", tx);

      const economics = await program.account.tokenEconomics.fetch(economicsAddress);
      assert.isFalse(economics.isPaused);

      console.log("▶️ 系统已恢复");
    });

    it("更新奖励率", async () => {
      console.log("📈 更新奖励率...");

      const newRate = new anchor.BN(200); // 200 tokens per second

      const tx = await program.methods
        .updateRewardRate(newRate)
        .accounts({
          authority: authority,
        })
        .rpc();

      console.log("✅ 更新奖励率交易签名:", tx);

      const economics = await program.account.tokenEconomics.fetch(economicsAddress);
      assert.equal(economics.rewardRatePerSecond.toString(), newRate.toString());

      console.log("📈 奖励率已更新为:", Number(economics.rewardRatePerSecond), "代币/秒");
    });
  });

  describe("📊 系统统计", () => {
    it("验证经济系统整体状态", async () => {
      console.log("📊 获取系统统计...");

      const economics = await program.account.tokenEconomics.fetch(economicsAddress);
      const pool = await program.account.stakingPool.fetch(stakingPoolAddress);
      const userStake = await program.account.userStake.fetch(userStakeAddress);

      console.log("\n📈 === 经济系统统计报告 ===");
      console.log("🏛️ 全局状态:");
      console.log("  - 总质押量:", Number(economics.totalStaked) / 10**9, "代币");
      console.log("  - 活跃质押者:", economics.activeStakers.toNumber(), "人");
      console.log("  - 已分发奖励:", Number(economics.totalRewardsDistributed) / 10**9, "代币");
      console.log("  - 当前奖励率:", economics.rewardRatePerSecond.toNumber(), "代币/秒");
      console.log("  - 治理费率:", economics.governanceFeeBps / 100, "%");

      console.log("\n🏊 池状态 (Pool", pool.poolId, "):");
      console.log("  - 池总质押:", Number(pool.totalStaked) / 10**9, "代币");
      console.log("  - 活跃质押者:", pool.activeStakers, "人");
      console.log("  - 奖励倍数:", pool.rewardMultiplier / 100, "x");
      console.log("  - 最大容量:", Number(pool.maxCapacity) / 10**9, "代币");
      console.log("  - 利用率:", (Number(pool.totalStaked) / Number(pool.maxCapacity) * 100).toFixed(2), "%");

      console.log("\n👤 用户状态:");
      console.log("  - 质押数量:", Number(userStake.totalStaked) / 10**9, "代币");
      console.log("  - 待领取奖励:", Number(userStake.pendingRewards) / 10**9, "代币");
      console.log("  - 已领取奖励:", Number(userStake.totalRewardsClaimed) / 10**9, "代币");
      console.log("  - 首次质押时间:", new Date(userStake.firstStakeTime.toNumber() * 1000).toLocaleString());

      // 验证数据一致性
      assert.isTrue(economics.totalStaked.toNumber() > 0);
      assert.isTrue(economics.activeStakers.toNumber() > 0);
      assert.isTrue(economics.totalRewardsDistributed.toNumber() > 0);
      assert.equal(pool.totalStaked.toString(), economics.totalStaked.toString());
      assert.equal(userStake.totalStaked.toString(), pool.totalStaked.toString());

      console.log("\n✅ 系统数据一致性验证通过");
    });
  });

  // 错误情况测试
  describe("🚨 错误处理测试", () => {
    it("尝试质押低于最小数量", async () => {
      console.log("❌ 测试最小质押数量限制...");

      const tooSmallAmount = new anchor.BN(100); // 小于最小值 1000

      try {
        await program.methods
          .stakeTokens(POOL_ID, tooSmallAmount)
          .accounts({
            user: authority,
          })
          .rpc();
        
        assert.fail("应该抛出错误");
      } catch (error) {
        console.log("✅ 正确拒绝了过小的质押数量");
        assert.include(error.toString(), "BelowMinimumStake");
      }
    });

    it("测试系统暂停时的操作限制", async () => {
      console.log("⏸️ 测试暂停状态限制...");

      // 先暂停系统
      await program.methods
        .setPauseState(true)
        .accounts({
          authority: authority,
        })
        .rpc();

      try {
        await program.methods
          .stakeTokens(POOL_ID, new anchor.BN(1000 * 10**9))
          .accounts({
            user: authority,
          })
          .rpc();
        
        assert.fail("应该抛出错误");
      } catch (error) {
        console.log("✅ 正确阻止了暂停状态下的质押");
        assert.include(error.toString(), "SystemPaused");
      }

      // 恢复系统
      await program.methods
        .setPauseState(false)
        .accounts({
          authority: authority,
        })
        .rpc();
    });
  });
});