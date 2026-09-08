use std::{fmt::Debug, path::Path};

use nx_common::{Readable, Writable};

pub fn dump<T>(path: impl AsRef<Path>, ctx: &mut T::ReadContext) -> color_eyre::Result<()>
where
    T: Readable + Debug + serde::Serialize,
{
    let object = T::read_file(path, ctx)?;
    let contents = ron::ser::to_string_pretty(&object, ron::ser::PrettyConfig::new())?;
    println!("{}", contents);

    Ok(())
}

pub fn round_trip<T>(
    input_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
    read_ctx: &mut T::ReadContext,
    write_ctx: &mut T::WriteContext,
) -> color_eyre::Result<()>
where
    T: Readable + Writable,
{
    T::read_file(input_path, read_ctx)?.write_file(output_path, write_ctx)?;
    Ok(())
}
