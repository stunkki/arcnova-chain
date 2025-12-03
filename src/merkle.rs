use sha2::{Sha256, Digest};

/// Hash a single leaf
pub fn hash_leaf(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Build a merkle root from a list of strings (transactions)
pub fn merkle_root(mut leaves: Vec<String>) -> String {
    if leaves.is_empty() {
        return "0".repeat(64); // empty tree
    }

    // Hash all leaves first
    leaves = leaves.into_iter().map(|l| hash_leaf(&l)).collect();

    while leaves.len() > 1 {
        let mut next_level = vec![];

        for i in (0..leaves.len()).step_by(2) {
            let left = &leaves[i];
            let right = if i + 1 < leaves.len() {
                &leaves[i + 1]
            } else {
                &leaves[i] // duplicate last if odd
            };

            let mut hasher = Sha256::new();
            hasher.update(left);
            hasher.update(right);

            next_level.push(format!("{:x}", hasher.finalize()));
        }

        leaves = next_level;
    }

    leaves[0].clone()
}