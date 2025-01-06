use clap::Parser;
use log::{debug, error, info, warn};
use std::{
    env::set_var,
    fs::File,
    io::{BufReader, BufWriter, Error, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value_t = false)]
    verbose: bool,
    input: PathBuf,
    output: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    match args.verbose {
        true => set_var("RUST_LOG", "debug"),
        false => set_var("RUST_LOG", "info"),
    }

    env_logger::init();

    let path = Path::new(&args.input);
    let output_path = match args.output {
        Some(path) => path,
        None => {
            let filename = path
                .file_stem()
                .expect("no filename")
                .to_str()
                .expect("invalid filename");
            let parent = path.parent();

            parent.unwrap().join(format!("{}_decoded.jpg", filename))
        }
    };

    decode_level_file(PathBuf::from(path), output_path)?;

    Ok(())
}

const CONSTANTS: [u32; 8] = [
    0xa37b9c37, 0x93553df1, 0x13719703, 0x71fc9e6d, 0x6311cc55, 0x55ee56be, 0xf7b9d5c3, 0xe9a09c77,
];

fn decode_level_file(input_path: PathBuf, output_path: PathBuf) -> Result<(), Error> {
    let input_file = File::open(&input_path)?;
    info!("Decoding file from {:?}", &input_path);

    let file_size_with_footer = input_file.metadata()?.len();
    let file_size = file_size_with_footer - 16;

    match file_size & 0x1f != 0 {
        true => error!("Invalid file size"), // continue anyway
        false => debug!("File size is valid"),
    }

    let mut reader = BufReader::new(input_file);
    debug!("Created reader");

    let output_file = File::create(&output_path)?;
    debug!("Created output file in {:?}", &output_path);

    let mut writer = BufWriter::new(output_file);
    debug!("Created writer");

    let mut buffer = [0; 32];

    let mut v8: u32 = 0;
    let mut v9: u32 = 0;

    loop {
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read < 32 {
            if bytes_read == 16 {
                debug!("Reached file footer");

                // some kind of additional verification, must always be -2 when decoding
                let flag = i32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                debug!("Flag: {}", flag);

                let expected_checksum =
                    -1 - i32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);

                let checksum = (file_size as u32) ^ v9 ^ v8;

                match expected_checksum == checksum as i32 {
                    true => debug!("Expected checksum matches actual checksum"),
                    false => {
                        warn!("Expected checksum does not match actual checksum");
                        debug!("Expected checksum: {}", expected_checksum);
                        debug!("Actual checksum: {}", checksum);
                    }
                }

                let expected_size =
                    -1 - i32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);

                match expected_size == file_size_with_footer as i32 {
                    true => debug!("Expected size matches actual size"),
                    false => {
                        warn!("Expected size does not match actual size");
                        debug!("Expected size: {}", expected_size);
                        debug!("Actual size: {}", file_size_with_footer);
                    }
                }

                writer.write_all(&buffer[..16])?;
            } else {
                warn!("Unexpected EOF");
            }

            break;
        }

        for i in 0..8 {
            let mut value = u32::from_le_bytes([
                buffer[i * 4],
                buffer[i * 4 + 1],
                buffer[i * 4 + 2],
                buffer[i * 4 + 3],
            ]);

            if i % 3 == 0 || i % 3 == 1 {
                v9 ^= value;
            } else {
                v8 ^= value;
            }

            value ^= CONSTANTS[i];
            writer.write_all(&value.to_le_bytes())?;
        }

        writer.flush()?;
    }

    info!("Decoded file to {:?}", &output_path);

    Ok(())
}
