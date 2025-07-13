# ✨ Sol-Vote 🗳️

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Built with Anchor](https://img.shields.io/badge/Built%20with-Anchor-blue)](https://www.anchor-lang.com/)
[![Solana](https://img.shields.io/badge/Solana-black?logo=solana)](https://solana.com/)

> A decentralized voting system built on the Solana blockchain using the Anchor framework. Sol-Vote allows users to create polls, cast votes, and view results in a transparent and trustless manner.

## ✅ Features

- 📝 **Create Polls**: Create polls with customizable titles, descriptions, and up to 5 options (modifiable in code)
- ⏱️ **Time-Limited Voting**: Set duration for how long a poll should remain open
- 🔒 **Secure Voting**: Each user can only vote once per poll
- 🔍 **Transparent Results**: All votes are recorded on-chain and visible to everyone

## 🔧 Technical Overview

Sol-Vote is built using:
- 🚀 [Solana](https://solana.com/) blockchain for secure and fast transactions
- ⚓ [Anchor Framework](https://www.anchor-lang.com/) for smart contract development
- 🦀 [Rust](https://www.rust-lang.org/) for on-chain program logic

## 📁 Project Structure

```
sol-vote/
├── programs/sol-vote/        # Solana program written in Rust
│   └── src/lib.rs            # Core smart contract logic
├── tests/                    # Integration tests
├── migrations/               # Deployment scripts
└── Anchor.toml               # Project configuration
```

## 🚀 Getting Started

### 📋 Prerequisites

- [Solana CLI tools](https://docs.solanalabs.com/cli/install)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/) (v14 or higher)
- [Rust](https://www.rust-lang.org/tools/install)

### 💻 Installation

1. Clone the repository
   ```bash
   git clone https://github.com/yourusername/sol-vote.git
   cd sol-vote
   ```

2. Install dependencies
   ```bash
   yarn install
   ```

3. Build the program
   ```bash
   anchor build
   ```

### 🧪 Running Tests

```bash
anchor test
```

### 🚢 Deployment

To deploy to a Solana cluster:

```bash
anchor deploy
```

## 📝 Usage

### 📊 Creating a Poll

```typescript
await program.methods
  .initialize(
    "Your Poll Title",
    "Description of your poll",
    ["Option 1", "Option 2", "Option 3"],
    new BN(86400) // Duration in seconds (24 hours)
  )
  .accounts({
    user: userWallet.publicKey,
    pollCounter: pollCounterPda,
    pollAccount: pollPda,
    systemProgram: SystemProgram.programId,
  })
  .signers([userWallet])
  .rpc();
```

### 🗳️ Casting a Vote

```typescript
await program.methods
  .castVote(optionIndex) // Index of the option you want to vote for
  .accounts({
    userVote: userVotePda,
    user: userWallet.publicKey,
    pollAccount: pollPda,
    systemProgram: SystemProgram.programId,
  })
  .signers([userWallet])
  .rpc();
```

## 🧩 Tests

Tests are written using Mocha and Chai for assertions. The test suite includes:

- ✅ Poll initialization verification
- ✅ Voting mechanism validation
- ✅ Poll data integrity checks

### Running Tests

```bash
anchor test
```

### Test Structure

The tests are located in the `tests/sol-vote.ts` file. They follow this structure:

1. Setup a new poll with test data
2. Verify poll initialization worked correctly
3. Cast a vote and verify the vote count updated

### Adding New Tests

When extending functionality, add corresponding tests in the test file. For example:

```typescript
it("validates poll duration constraints", async () => {
  // Test code for duration validation
});
```

Comprehensive tests ensure your smart contract operates correctly and securely on the Solana blockchain.

## 🤝 Contributing

Contributions are welcome! Here's how you can help improve Sol-Vote:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add some amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

Please make sure to update tests as appropriate and follow the existing code style.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

---

<div align="center">
  <img src="https://solana.com/src/img/branding/solanaLogoMark.svg" alt="Solana Logo" width="40" height="40">
  <p>Made with ❤️ for the Solana community</p>
  <p>⭐ Star this repository if you found it useful! ⭐</p>
</div>
