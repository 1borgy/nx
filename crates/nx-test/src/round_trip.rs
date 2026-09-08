use std::{
    fs,
    io::{BufReader, Cursor, Read},
    path::{Path, PathBuf},
};

use anyhow::anyhow;

use nx_common::{Readable, Writable};

fn diff_bytes(input_bytes: &[u8], output_bytes: &[u8]) -> anyhow::Result<()> {
    if input_bytes.len() == output_bytes.len() {
        let mut num_diff_bytes = 0;

        for (input_byte, output_byte) in input_bytes.iter().zip(output_bytes.iter()) {
            if input_byte != output_byte {
                num_diff_bytes += 1;
            }
        }

        if num_diff_bytes == 0 {
            Ok(())
        } else {
            Err(anyhow!("{} bytes different", num_diff_bytes))
        }
    } else {
        Err(anyhow!(
            "input size ({}) does not match output size ({})",
            input_bytes.len(),
            output_bytes.len()
        ))
    }
}

fn round_trip_one<T>(
    path: impl AsRef<Path>,
    read_ctx: &mut T::ReadContext,
    write_ctx: &mut T::WriteContext,
) -> anyhow::Result<()>
where
    T: Readable + Writable,
{
    let path = path.as_ref();

    let in_file = fs::File::open(&path)?;
    let mut reader = BufReader::new(in_file);
    let mut input_bytes = Vec::new();
    reader.read_to_end(&mut input_bytes)?;
    let mut input_cursor = Cursor::new(&input_bytes);

    let object = T::read(&mut input_cursor, read_ctx)?;

    let mut output_bytes = Vec::new();
    let mut output_cursor = Cursor::new(&mut output_bytes);
    object.write(&mut output_cursor, write_ctx)?;

    diff_bytes(&input_bytes, &output_bytes).map_err(|err| anyhow!("round_trip_entry: {}", err))
}

fn test_one<T>(
    path: impl AsRef<Path>,
    read_ctx: &mut T::ReadContext,
    write_ctx: &mut T::WriteContext,
) -> anyhow::Result<()>
where
    T: Readable + Writable,
{
    let path = path.as_ref();

    let mut errors = Vec::new();

    for result in [round_trip_one::<T>(path, read_ctx, write_ctx)] {
        match result {
            Ok(_) => {}
            Err(err) => errors.push(err),
        }
    }

    if !errors.is_empty() {
        let failures = errors
            .into_iter()
            .map(|e| format!("  {}", e))
            .collect::<Vec<_>>()
            .join("\r\n");

        Err(anyhow!("[{}]\n{}", path.display(), failures))
    } else {
        Ok(())
    }
}

pub async fn test_round_trip<T>(
    paths: impl IntoIterator<Item = PathBuf>,
    read_ctx: &mut T::ReadContext,
    write_ctx: &mut T::WriteContext,
) where
    T: Readable + Writable,
{
    let mut handles = Vec::new();
    let paths = paths.into_iter().collect::<Vec<_>>();

    for path in paths.iter() {
        let path = path.clone();
        let mut read_ctx = read_ctx.clone();
        let mut write_ctx = write_ctx.clone();
        handles.push(tokio::task::spawn_blocking(move || {
            test_one::<T>(path, &mut read_ctx, &mut write_ctx)
        }))
    }

    let mut errors = Vec::new();
    for (handle, path) in handles.into_iter().zip(paths) {
        match handle.await {
            Ok(result) => match result {
                Ok(_) => {}
                Err(err) => errors.push(err),
            },
            Err(err) => errors.push(anyhow!("[{}]\n  {}", path.display(), err)),
        }
    }

    assert!(
        errors.is_empty(),
        "\n{}\n",
        errors
            .into_iter()
            .map(|e| format!("{}", e))
            .collect::<Vec<_>>()
            .join("\n\n")
    )
}
