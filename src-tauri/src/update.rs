use reqwest;
use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    prerelease: bool,
}

pub fn check_for_update(
    current: Version,
    repo_owner: &str,
    repo_name: &str,
) -> Result<Option<(Version, String)>, String> {
    let releases_url = format!(
        "https://api.github.com/repos/{}/{}/releases",
        repo_owner, repo_name
    );

    println!("Checking for updates at {}", releases_url);
    println!("Current Version: {}", current);

    // Fetch releases from GitHub API
    let client = reqwest::blocking::Client::builder()
        .user_agent("update-checker")
        .build()
        .map_err(|e| e.to_string())?;
    let releases = client
        .get(&releases_url)
        .send()
        .map_err(|e| e.to_string())?;
    println!("Releases: {:?}", releases);

    let r: Vec<GithubRelease> = releases.json().map_err(|e| e.to_string())?;

    // Find newest non-prerelease version
    let newest = r
        .into_iter()
        .filter(|release| !release.prerelease)
        .filter_map(|release| {
            let version_str = release.tag_name.splitn(2, 'v').last().unwrap_or_default();
            match Version::parse(version_str) {
                Ok(version) => Some((version, release.html_url)),
                Err(_) => None,
            }
        })
        .max_by(|(v1, _), (v2, _)| v1.cmp(v2));

    println!("newst: {:?}", newest);
    // Return newer version if found
    Ok(match &newest {
        Some((version, ref url)) if version > &current => Some((version.clone(), url.clone())),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_for_update() {
        let current = semver::Version::parse("0.1.0").unwrap();
        let repo_owner = "blopker";
        let repo_name = "alic";
        let result = check_for_update(current, repo_owner, repo_name);
        println!("Result: {:?}", result);
        assert!(result.is_ok());
    }
}
