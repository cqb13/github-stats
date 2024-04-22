pub fn all(owner: String, repo: String) -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get(format!("https://api.github.com/repos/{}/{}", owner, repo))?
        .text()?;

    println!("body = {body:?}");
    Ok(())
}
