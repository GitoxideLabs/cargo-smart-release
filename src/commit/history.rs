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
        // The expected size used to be for the *test* build during the gix 0.83 adaptation:
        // with `gix-testtools` and `gix` resolving the same `gix-hash` version, Cargo feature
        // unification enabled `gix-hash/sha256`, making `gix::ObjectId` its wider
        // SHA-1/SHA-256 enum. With gix 0.86 they can resolve different `gix-hash` versions, so
        // the main `gix` package follows this crate's explicit production configuration of
        // `gix/sha1` only. Growth from either our fields or gix's types trips this deliberately,
        // so the bump can be reviewed.
        let object_id_size = std::mem::size_of::<gix::ObjectId>();
        let item_size = std::mem::size_of::<Item>();
        const SHA1_ONLY_LAYOUT: (usize, usize) = (20, 200);
        const DUAL_HASH_LAYOUT: (usize, usize) = (33, 240);
        assert!(
            matches!(
                (object_id_size, item_size),
                SHA1_ONLY_LAYOUT | DUAL_HASH_LAYOUT
            ),
            "unexpected sizes: gix::ObjectId={object_id_size}, Item={item_size}; expected SHA-1-only ({}, {}) or SHA-1/SHA-256 ({}, {})",
            SHA1_ONLY_LAYOUT.0,
            SHA1_ONLY_LAYOUT.1,
            DUAL_HASH_LAYOUT.0,
            DUAL_HASH_LAYOUT.1
        )
    }
}
