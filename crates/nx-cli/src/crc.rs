#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Generate { value: String },
}

pub fn main(command: Command) -> color_eyre::Result<()> {
    match command {
        Command::Generate { value } => {
            let value = value.to_lowercase();
            let checksum = nx_crc::checksum(&value.as_bytes().to_vec());
            println!("name: {}", value);
            println!("hex: {:#08x}", checksum);
            println!("decimal: {}", checksum);
            Ok(())
        }
    }
}
