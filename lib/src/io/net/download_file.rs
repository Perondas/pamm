use std::fs::File;
use std::path::Path;
use url::Url;

pub(crate) fn download_file(
    file_path: &Path,
    file_url: Url,
    expected_len: u64,
) -> anyhow::Result<()> {
    let resp = ureq::get(file_url.to_string()).call()?;
    let body = resp.into_body();

    let mut file = File::create(file_path)?;
    let actual_len = std::io::copy(&mut body.into_reader(), &mut file)?;

    if actual_len != expected_len {
        anyhow::bail!(
            "Downloaded file length {} does not match expected length {} for file {}",
            actual_len,
            expected_len,
            file_url
        );
    }
    Ok(())
}
