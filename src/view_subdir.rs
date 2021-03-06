use super::View;
use super::replace_subtree;
use git2::*;
use std::path::Path;
use std::path::PathBuf;

pub struct SubdirView
{
    subdir: PathBuf,
}

impl SubdirView
{
    pub fn new(subdir: &Path) -> SubdirView
    {
        SubdirView {
            subdir: subdir.to_path_buf(),
        }
    }
}

impl View for SubdirView
{
    fn apply(&self, tree: &Tree) -> Option<Oid>
    {
        tree.get_path(&self.subdir).map(|x| x.id()).ok()
    }

    fn unapply(&self, repo: &Repository, tree: &Tree, parent_tree: &Tree) -> Oid
    {
        replace_subtree(&repo, &self.subdir, &tree, &parent_tree)
    }
}
