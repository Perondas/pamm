use pamm_lib::application::pack::write::delete_pack_content::DeletePackContent;
use pamm_lib::application::repo::client::ClientRepoHandle;
use std::path::Path;

#[err_wrap]
pub async fn delete_pack_content(repo_path: String, pack_name: String) -> anyhow::Result<()> {
    let repo_path = Path::new(&repo_path);

    let mut handle = ClientRepoHandle::open(repo_path).await?;

    handle.delete_pack_content(&pack_name).await?;

    Ok(())
}
