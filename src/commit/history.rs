use crate::commit::Message;

/// A head reference will all commits that are 'governed' by it, that is are in its exclusive ancestry.
pub struct Segment<'a> {
    pub head: gix::refs::Reference,
    /// only relevant history items, that is those that change code in the respective crate.
    pub history: Vec<&'a Item>,
}

pub struct Item {
    pub id: gix::ObjectId,
    pub message: Message,
    pub commit_time: gix::date::Time,
    pub tree_id: gix::ObjectId,
    pub parent_tree_id: Option<gix::ObjectId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_item() {
        // The expected size is for the *test* build: the `gix-testtools` dev-dependency
        // enables `gix-hash/sha256`, so `gix::ObjectId` is its wider SHA-1/SHA-256 enum
        // (33 bytes) here and `Item` holds three ids; a production (sha1-only) build is
        // ~200. Growth from either our fields or gix's types trips this deliberately, so
        // the bump can be reviewed.
        assert_eq!(
            std::mem::size_of::<Item>(),
            240,
            "there are plenty of these loaded at a time and we should not let it grow unnoticed."
        )
    }
}
