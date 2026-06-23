# PQC-UOV

## Instructions

### Prerequisites
Having Rust with Cargo installed  

### Build

Run the following command to build the project 
```bash
cargo build --release
```

### Tests

To run all tests execute the following command
```bash
cargo test
```
To run the measurement test with output
```bash
cargo test --release --test timing -- --nocapture
```
To run with cycles use this:
```bash
cargo test --release --test cycleTiming -- --nocapture
```

## GF(16) Elements
| Decimal | Binary   | Polynomial          |
|---------|----------|---------------------|
| 0       | `0b0000` | `0`                 |
| 1       | `0b0001` | `1`                 |
| 2       | `0b0010` | `x`                 |
| 3       | `0b0011` | `x + 1`             |
| 4       | `0b0100` | `x²`                |
| 5       | `0b0101` | `x² + 1`            |
| 6       | `0b0110` | `x² + x`            |
| 7       | `0b0111` | `x² + x + 1`        |
| 8       | `0b1000` | `x³`                |
| 9       | `0b1001` | `x³ + 1`            |
| 10      | `0b1010` | `x³ + x`            |
| 11      | `0b1011` | `x³ + x + 1`        |
| 12      | `0b1100` | `x³ + x²`           |
| 13      | `0b1101` | `x³ + x² + 1`       |
| 14      | `0b1110` | `x³ + x² + x`       |
| 15      | `0b1111` | `x³ + x² + x + 1`   |

### Mapping from bit to polynomial
| bit 3 | bit 2 | bit 1 | bit 0 |
|-------|-------|-------|-------|
| `x³`  | `x²`  | `x`   | `1`   |


## Noter
[Overleaf Notes](https://da.overleaf.com/9352114841ttkqjcycdxjt#287b7b)

## Præsentation
[Google Slides Presentation](https://docs.google.com/presentation/d/1yGwOB2s1RrJXixD5SoTRisX-yOD5LKnJKlnM27CWnkY/edit?usp=sharing)
