# hash-omikuji

SHA-256 based deterministic fortune telling CLI.

**This command can only be executed on January 1st!**

## Usage

```bash
npx hash-omikuji
```

### Options

- `--force-year <YYYY>` - Force execution for a specific year (bypasses January 1st restriction)
- `--seed <string>` - Custom seed (default: device fingerprint from username@hostname)
- `--json` - Output as JSON
- `--short` - Show only top 5 luck scores
- `--show-seed` - Display seed and fingerprint

### Example Output

```
🎍 Hash-Omikuji 2026 🎍

Lucky Number      : 165
Lucky Hex         : 0xBF
Lucky Color       : #BFA532
Lucky Bits        : 0010 0101 0011 0100

Lucky Day         : 2026-09-01 (244 / 365)
Lucky Time        : 02:40

Lucky Power of 2  : 64
Lucky ASCII       : 'K'
Lucky Logic Gate  : XOR
Lucky Emoji       : 🎉

Luck Scores :
WiFi Luck         :  95 (Excellent)
Study Luck        :  94 (Excellent)
...

device:a3f2b1c9 | 5fe36d472c00c87d | 0xF01
```

## How It Works

Uses SHA-256 hash of `{year}-{seed}-{salt}` to deterministically generate:

- Lucky numbers, hex, color, bits
- Lucky day and time
- Lucky power of 2, ASCII, logic gate, emoji
- 16 luck categories with scores

Same input always produces the same output.

### Bit Layout (256-bit SHA-256)

```mermaid
packet-beta
0-7: "Lucky Number (8)"
8-15: "Lucky Hex (8)"
16-31: "Lucky Bits (16)"
32-40: "Lucky Day (9)"
41-45: "Hour (5)"
46-51: "Min (6)"
52-54: "Power of 2 (3)"
55-61: "ASCII (7)"
62-64: "Logic Gate (3)"
65-192: "Luck Scores (128)"
193-204: "Entropy (12)"
205-207: "Emoji (3)"
208-255: "Reserved (48)"
```

| Field | Bits | Range |
|-------|------|-------|
| Lucky Number | 0-7 (8bit) | 0-255 |
| Lucky Hex | 8-15 (8bit) | 0x00-0xFF |
| Lucky Bits | 16-31 (16bit) | display pattern |
| Lucky Day | 32-40 (9bit) | 1-365 |
| Lucky Hour | 41-45 (5bit) | 0-23 |
| Lucky Minute | 46-51 (6bit) | 0-59 |
| Lucky Power of 2 | 52-54 (3bit) | 2^n (1,2,4,8,16,32,64,128) |
| Lucky ASCII | 55-61 (7bit) | printable ASCII (32-126) |
| Lucky Logic Gate | 62-64 (3bit) | AND,OR,XOR,NOT,NAND,NOR,XNOR,BUFFER |
| Luck Scores | 65-192 (128bit) | 16 × 8bit scores |
| Entropy | 193-204 (12bit) | checksum display |
| Lucky Emoji | 205-207 (3bit) | 😀😎🎉🔥⭐💡🎯🚀 |
| Reserved | 208-255 (48bit) | unused |

## Build from Source

```bash
cd rust
cargo build --release
```

## License

MIT
