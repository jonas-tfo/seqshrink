
use clap::{Parser, Subcommand};
use compression_btk::utils::fileutils::{decode_fasta_compressed, parse_fasta, save_fasta_compressed, write_fasta};

  #[derive(Parser)]
  struct Args {
      #[command(subcommand)]
      command: Command,
  }

  #[derive(Subcommand)]
  enum Command {
      Compress {
          #[arg(short, long)] input: String,
          #[arg(short, long)] output: String,
          #[arg(short, long)] mode: String,
      },
      Decompress {
          #[arg(short, long)] input: String,
          #[arg(short, long)] output: String,
      },
  }


fn mode_from_str(s: &str) -> Option<u8> {
    match s {
        "dna2" => Some(0),
        "dna3" => Some(1),
        "rna2" => Some(2),
        "rna3" => Some(3),
        "prot5" => Some(4),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();

  match args.command {
      Command::Compress { input, output, mode } => {
          let mode_num = mode_from_str(&mode).ok_or_else(|| format!("unknown mode: {mode}"))?;
          let (ids, sequences) = parse_fasta(&input)?;
          save_fasta_compressed(&output, mode_num, ids, sequences)?;
      }
      Command::Decompress { input, output } => {
          let (ids, seqs) = decode_fasta_compressed(&input)?;
          write_fasta(&ids, &seqs, &output)?;
      }
  }

  Ok(())
}

