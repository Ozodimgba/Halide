# Halide

Solana developer experience that doesn't suck.

## The Problem

Let's be real, Solana development can be a pain in the ass sometimes:

- Transaction failures with cryptic error messages that make you want to pull your hair out
- Testing that's slower than a snail on tranquilizers esp on my [insert non mac device]
- Deploying programs feels like you need a PhD in Solana internals
- Trying to test CPIs? Good luck setting up that environment

I once wasted 3 days debugging a transaction that failed with `0x1` as the only error. THREE. DAYS. I'm better at solana stuff now but the lads should have training-wheels

Oh, and testing my program that calls Metaplex? Had to set up a local validator, which takes forever to start, only for it to fail because of some obscure configuration issue.

## What This Actually Does

`halide` is a CLI I built because I got tired of this crap:

```bash
# Deploy your program without the headache(wtf are buffers, and where did all my SOL go, 3 SOL! gone!)
halide deploy program.so

# Monitor ALL transactions for any program - see exactly why they're failing - just like prometheus for your backend
halide monitor Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS

# Run fast tests with LiteSVM and all the programs you actually need
halide spin litezes
```

## Features

### Super Fast Testing with LiteSVM

- Tests run in milliseconds, not minutes
- Metaplex, SPL and other common programs pre-loaded
- Test CPIs without wanting to throw your laptop out the window
- No need to start a validator for simple tests

```typescript
// Example test that would normally be a nightmare to set up
import { setupTest, loadProgram, loadMetaplex } from 'halide';

describe('My Program', () => {
  const env = setupTest();
  
  it('can mint an NFT', async () => {
    // This just works™ because Metaplex is already loaded
    const result = await myProgram.mintNFT({...});
    assert(result.success);
  });
});
```

### Transaction Monitoring That Actually Helps

Stop guessing why your transactions are failing:

- See the exact instruction that failed
- Get human-readable error messages
- Track all transactions for your program without setting up infrastructure
- Filter by account, transaction type, or time range

### One-Command Deployments

Just `halide deploy` and you're done. No more:
- Juggling BPF loaders
- Managing program upgrades
- Figuring out which network you're on
- Dealing with insufficient funds
- Squads Multisig authority program upgrade support -- hmm who needs this

## Installation

chill a bit

## Who Is This For?

- Solana devs who value their time and sanity
- Teams who want to ship faster
- Anyone who's muttered "why is this so complicated?" while working with Solana

## Current Status

This is an early version that I'm using for myself. There might rough edges, but if it saves me hours it's a tool alright. PRs welcome!

## License

MIT - do whatever you want with it
