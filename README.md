# Compression-BTK

A CLI tool for compressing biological sequence data (FASTA format) using bit-packing tailored to each alphabet type.

## Compression modes

| Mode    | Alphabet                | Bits/symbol |
|---------|-------------------------|-------------|
| `dna2`  | A, C, G, T              | 2           |
| `dna3`  | A, C, G, T, N, Gap      | 3           |
| `rna2`  | A, C, G, U              | 2           |
| `rna3`  | A, C, G, U, N, Gap      | 3           |
| `prot5` | 20 amino acids, X, Stop | 5           |

Sequence IDs are stored separately and compressed with DEFLATE. The output format uses a custom binary container (`BLOK`).

## Usage

```sh
# Compress
compression-btk compress -i input.fasta -o output.blok -m dna2

# Decompress
compression-btk decompress -i output.blok -o recovered.fasta
```

