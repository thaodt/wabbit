use std::collections::HashMap;
use typed_arena::Arena;

pub struct Scope<'a, T: 'a> {
    values: HashMap<String, T>,
    parent: Option<&'a Scope<'a, T>>,
}

pub struct Environment<'a, T: 'a> {
    current_scope: &'a Scope<'a, T>,
    arena: &'a Arena<Scope<'a, T>>,
}

impl<'a, T> Environment<'a, T> {
    pub fn new(arena: &'a Arena<Scope<'a, T>>) -> Self {
        let root_scope = arena.alloc(Scope {
            values: HashMap::new(),
            parent: None,
        });
        Self {
            current_scope: root_scope,
            arena,
        }
    }

    pub fn enter_scope(&mut self) {
        let new_scope = self.arena.alloc(Scope {
            values: HashMap::new(),
            parent: Some(self.current_scope),
        });
        self.current_scope = new_scope;
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.current_scope.parent {
            self.current_scope = parent;
        }
    }
}
