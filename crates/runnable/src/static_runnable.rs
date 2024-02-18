//! Definitions of runnables with a static file config definition, not dependent on the application state.

use std::path::PathBuf;

use crate::{static_source::Definition, Runnable, RunnableId, SpawnInTerminal};

/// A single config file entry with the deserialized runnable definition.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticRunnable {
    id: RunnableId,
    definition: Definition,
}

impl StaticRunnable {
    pub(super) fn new(id: usize, runnable: Definition) -> Self {
        Self {
            id: RunnableId(format!("static_{}_{}", runnable.label, id)),
            definition: runnable,
        }
    }
}

impl Runnable for StaticRunnable {
    fn exec(&self, cwd: Option<PathBuf>) -> Option<SpawnInTerminal> {
        Some(SpawnInTerminal {
            id: self.id.clone(),
            use_new_terminal: self.definition.use_new_terminal,
            allow_multiple: self.definition.allow_multiple,
            label: self.definition.label.clone(),
            command: self.definition.command.clone(),
            args: self.definition.args.clone(),
            cwd: self.definition.cwd.clone().or(cwd),
            env: self.definition.env.clone(),
        })
    }

    fn name(&self) -> &str {
        &self.definition.label
    }

    fn id(&self) -> &RunnableId {
        &self.id
    }
}
