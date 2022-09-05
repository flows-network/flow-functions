use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Branchroot {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub ref_type: String,
    pub master_branch: String,
    pub description: String,
    pub pusher_type: String,
    pub repository: Repository,
    pub sender: Doer,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub email: String,
    pub name: String,
    pub username: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: Doer,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    #[serde(default)]
    pub homepage: String,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: String,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: i64,
    #[serde(default)]
    pub mirror_url: String,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: i64,
    pub license: License,
    pub allow_forking: bool,
    pub is_template: bool,
    pub web_commit_signoff_required: bool,
    #[serde(default)]
    pub topics: Vec<Value>,
    pub visibility: String,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub default_branch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Doer {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    #[serde(default)]
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    key: String,
    name: String,
    spdx_id: String,
    url: String,
    node_id: String,
}


#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let result: Result<Branchroot> = serde_json::from_str(s.as_str());

    if let Ok(payload) = result {
        return payload.sender.html_url.to_string();
    }

    "failed to parse to Json".to_string()
}

