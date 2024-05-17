use serde::{Deserialize, Serialize};
use crate::generators::git::github::Github;
use crate::generators::git::gitlab::Gitlab;


#[derive(Serialize, Deserialize)]
pub enum GitTarget {
    Gitlab(Gitlab),
    Github(Github),
}

#[derive(Serialize, Deserialize)]
pub struct CiCd {
    pub git_target: GitTarget,
}
impl CiCd {
    pub fn new(git_target: GitTarget) -> Self {
        CiCd {
            git_target,
        }
    }
}
