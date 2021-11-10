use crate::{BlockId, DeclId};

/// Collection of definitions that can be exported from a module
#[derive(Debug, Clone)]
pub struct Overlay {
    pub decls: Vec<(Vec<u8>, DeclId)>,
    pub env_vars: Vec<(Vec<u8>, BlockId)>,
}

impl Overlay {
    pub fn new() -> Self {
        Overlay {
            decls: vec![],
            env_vars: vec![],
        }
    }

    pub fn add_decl(&mut self, name: &[u8], decl_id: DeclId) {
        self.decls.push((name.to_vec(), decl_id));
    }

    pub fn add_env_var(&mut self, name: &[u8], block_id: BlockId) {
        self.env_vars.push((name.to_vec(), block_id));
    }

    pub fn extend(&mut self, other: &Overlay) {
        self.decls.extend(other.decls.clone());
        self.env_vars.extend(other.env_vars.clone());
    }

    pub fn is_empty(&self) -> bool {
        self.decls.is_empty() || self.env_vars.is_empty()
    }

    pub fn filtered(&self, name: &[u8]) -> Self {
        let mut result = Overlay::new();

        // for name in names {
            let decls = self
                .decls
                .iter()
                .filter_map(|decl| {
                    if &decl.0 == name {
                        Some((decl.0.clone(), decl.1))
                    } else {
                        None
                    }
                })
                .collect();

            let env_vars = self
                .env_vars
                .iter()
                .filter_map(|env_var| {
                    if &env_var.0 == name {
                        Some((env_var.0.clone(), env_var.1))
                    } else {
                        None
                    }
                })
                .collect();

            result.extend(&Overlay { decls, env_vars });
        // }

        result
    }

    pub fn with_head(&self, head: &[u8]) -> Self {
        let decls = self.decls.iter().map(|(name, id)| {
            let mut new_name = head.to_vec();
            new_name.push(b' ');
            new_name.extend(name);
            (new_name, *id)
        }).collect();

        let env_vars = self.env_vars.iter().map(|(name, id)| {
            let mut new_name = head.to_vec();
            new_name.push(b' ');
            new_name.extend(name);
            (new_name, *id)
        }).collect();

        Overlay { decls, env_vars }
    }
}
