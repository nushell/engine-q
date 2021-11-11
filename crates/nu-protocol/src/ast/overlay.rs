use crate::{BlockId, DeclId};

use std::collections::HashMap;

/// Collection of definitions that can be exported from a module
#[derive(Debug, Clone)]
pub struct Overlay {
    pub decls: HashMap<Vec<u8>, DeclId>,
    pub env_vars: HashMap<Vec<u8>, BlockId>,
}

impl Overlay {
    pub fn new() -> Self {
        Overlay {
            decls: HashMap::new(),
            env_vars: HashMap::new(),
        }
    }

    pub fn add_decl(&mut self, name: &[u8], decl_id: DeclId) -> Option<DeclId> {
        self.decls.insert(name.to_vec(), decl_id)
    }

    pub fn add_env_var(&mut self, name: &[u8], block_id: BlockId) -> Option<BlockId> {
        self.env_vars.insert(name.to_vec(), block_id)
    }

    pub fn extend(&mut self, other: &Overlay) {
        self.decls.extend(other.decls.clone());
        self.env_vars.extend(other.env_vars.clone());
    }

    pub fn is_empty(&self) -> bool {
        self.decls.is_empty() && self.env_vars.is_empty()
    }

    pub fn get_decl_id(&self, name: &[u8]) -> Option<DeclId> {
        self.decls.get(name).copied()
    }

    pub fn decl_with_head(&self, name: &[u8], head: &[u8]) -> Option<(Vec<u8>, DeclId)> {
        if let Some(id) = self.get_decl_id(name) {
            let mut new_name = head.to_vec();
            new_name.push(b' ');
            new_name.extend(name);
            Some((new_name, id))
        } else {
            None
        }
    }

    pub fn decls_with_head(&self, head: &[u8]) -> Vec<(Vec<u8>, DeclId)> {
        self.decls
            .iter()
            .map(|(name, id)| {
                let mut new_name = head.to_vec();
                new_name.push(b' ');
                new_name.extend(name);
                (new_name, *id)
            })
            .collect()
    }

    pub fn decls(&self) -> Vec<(Vec<u8>, DeclId)> {
        self.decls
            .iter()
            .map(|(name, id)| (name.clone(), *id))
            .collect()
    }
}

impl Default for Overlay {
    fn default() -> Self {
        Self::new()
    }
}
