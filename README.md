# sha-omikuji

SHA-256 based deterministic fortune telling CLI.

**This command can only be executed on January 1st!**

## Usage

```bash
npx sha-omikuji
```

### Options

- `--year <YYYY>` - Target year (default: current year)
- `--seed <string>` - Custom seed (default: username@hostname)
- `--json` - Output as JSON
- `--short` - Show only top 5 luck scores
- `--show-seed` - Display seed and fingerprint
- `--force` - Execute outside January 1st (with warning)

### Example Output

```
🎍 SHA-Omikuji 2026 🎍

Lucky Number      : 165
Lucky Hex         : 0xBF
Lucky Color       : #BFA532
Lucky Bits        : 0010 0101 0011 0100

Lucky Day         : 2026-09-01 (244 / 365)
Lucky Time        : 02:40

Active Luck Flags :
✖ Life  ✖ Health  ✔ Wealth  ✔ Career  ...

Luck Scores :
WiFi Luck         :  95 (Excellent)
Study Luck        :  94 (Excellent)
...

Entropy Check     : OK (0xF01)

[ Omikuji Art ]
SE##############
```

## How It Works

Uses SHA-256 hash of `{year}-{seed}-{salt}` to deterministically generate:

- Lucky numbers, hex, color, bits
- Lucky day and time
- 16 luck categories with scores and active flags
- Visual fingerprint (Omikuji Art)

Same input always produces the same output.

## Build from Source

```bash
cd rust
cargo build --release
```

## License

MIT
