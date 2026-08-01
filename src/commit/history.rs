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
        // `Item` holds three `gix::ObjectId`s, so its size depends on which hash kinds `gix-hash` was
        // built with: 200 bytes when `ObjectId` is SHA-1 only, 240 when it is the wider SHA-1/SHA-256
        // enum. Both are expected. This crate enables `gix/sha1` alone, but the `gix-testtools`
        // dev-dependency enables `gix-hash/sha256`, so whenever `gix` and `gix-testtools` resolve the
        // *same* `gix-hash`, Cargo's feature unification widens the ids in test builds only. That held
        // for gix 0.83 through 0.85, and is where 240 came from; since gix 0.86 the two resolve
        // different `gix-hash` versions, so test builds see the SHA-1-only ids production always had.
        // Each configuration still pins `Item` exactly, so growth from our own fields or from gix's
        // types trips this deliberately and the bump can be reviewed.
        let expected_size = match std::mem::size_of::<gix::ObjectId>() {
            20 => 200,
            33 => 240,
            unexpected => panic!("`gix::ObjectId` is neither SHA-1-only nor SHA-1/SHA-256 sized: {unexpected}"),
        };
        assert_eq!(
            std::mem::size_of::<Item>(),
            expected_size,
            "there are plenty of these loaded at a time and we should not let it grow unnoticed."
        )
    }
}
