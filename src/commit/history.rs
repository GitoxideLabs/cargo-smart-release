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
        let expected = match std::mem::size_of::<gix::ObjectId>() {
            20 => 200, // SHA-1 only
            33 => 240, // SHA-1 and SHA-256
            actual => panic!("unexpected ObjectId size: {actual}"),
        };
        assert_eq!(
            std::mem::size_of::<Item>(),
            expected,
            "there are plenty of these loaded at a time and we should not let it grow unnoticed."
        )
    }
}
