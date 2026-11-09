use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::writing::delete_pack_content::DeletePackContent;
use std::path::Path;

pub async fn delete_pack_content(repo_path: String, pack_name: String) -> anyhow::Result<()> {
    let repo_path = Path::new(&repo_path);

    let mut handle = ClientRepoHandle::open(repo_path).await?;

    handle.delete_pack_content(&pack_name).await?;

    Ok(())
}
