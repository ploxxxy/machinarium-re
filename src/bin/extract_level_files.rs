use clap::Parser;
use log::{debug, error, info};
use std::{
    env::set_var,
    fs::File,
    io::{BufReader, Cursor, Error, Read, Seek, SeekFrom},
    panic,
    path::{Path, PathBuf},
};

const HEADER_SIZE: usize = 48 * 1024;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value_t = false)]
    verbose: bool,
    input: PathBuf,
    output: PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    match args.verbose {
        true => set_var("RUST_LOG", "debug"),
        false => set_var("RUST_LOG", "info"),
    }

    env_logger::init();

    let path = Path::new(&args.input);
    let output_path = args.output;

    if !output_path.is_dir() {
        panic!("Output path must be a directory");
    }

    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let pointers = process_header(&mut reader)?;

    extract_level_file(&mut reader, pointers, &output_path)?;

    info!("Finished extracting to {:?}", output_path);
    
    Ok(())
}

struct Pointer {
    unknown: u32,
    offset: u32,
    size: u32,
}

fn process_header(reader: &mut BufReader<File>) -> Result<Vec<Pointer>, Error> {
    info!("Processing header...");

    let mut buffer = [0; 1];
    let mut pointers = Vec::<Pointer>::new();

    loop {
        let current_position = reader.seek(SeekFrom::Current(0))?;
        if current_position >= HEADER_SIZE as u64 {
            debug!("Reached end of header");
            break;
        }

        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            error!("Unexpected EOF");
            break;
        }

        if buffer[0] == 0 {
            continue; // skip all zero bytes
        }

        // go back before reading the values
        reader.seek(SeekFrom::Current(-1))?;

        let mut u32_buffer = [0; 12];
        reader.read_exact(&mut u32_buffer)?;

        reader.seek(SeekFrom::Current(12))?;

        let u32_values = u32_buffer
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<u32>>();

        let pointer = Pointer {
            unknown: u32_values[0],
            offset: u32_values[1],
            size: u32_values[2],
        };

        debug!(
            "Found pointer to {:x}, size of {}",
            pointer.offset, pointer.size
        );
        pointers.push(pointer);
    }

    info!("Found {} pointers", pointers.len());
    Ok(pointers)
}

fn extract_level_file(
    reader: &mut BufReader<File>,
    pointers: Vec<Pointer>,
    output_path: &PathBuf,
) -> Result<(), Error> {
    info!("Extracting level files...");

    for pointer in pointers {
        reader.seek(SeekFrom::Start(pointer.offset as u64))?;

        debug!("Reading {} bytes at {:x}", pointer.size, pointer.offset);
        
        let mut buffer = vec![0; pointer.size as usize];
        reader.read_exact(&mut buffer)?;

        let mut cursor = Cursor::new(buffer);

        let path = output_path.join(format!("level_{}.bin", pointer.offset));

        debug!("Writing file to {:?}", path);

        let mut output_file = File::create(path)?;
        std::io::copy(&mut cursor, &mut output_file)?;
    }

    Ok(())
}
