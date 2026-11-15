//! Tasks spawned by handlers and by other tasks, scoped to one session.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::future::Future;
use std::rc::{Rc, Weak};

use glommio::task::JoinHandle;

/// The tasks running in one session. They are cancelled when it ends.
#[derive(Default)]
pub struct Tasks {
    running: RefCell<HashMap<u64, JoinHandle<()>>>,
    next_id: Cell<u64>,
    /// Set once the session ended: nothing new is spawned.
    closed: Cell<bool>,
}

impl Tasks {
    /// Runs `task` next to the pumps until it finishes, is stopped through
    /// the returned handle, or the session ends.
    pub fn spawn(self: &Rc<Self>, task: impl Future<Output = ()> + 'static) -> TaskHandle {
        let id = self.next_id.get();
        self.next_id.set(id + 1);

        let handle = TaskHandle { id, tasks: Rc::downgrade(self) };
        if self.closed.get() {
            return handle;
        }

        // A finished task removes itself, so a long session does not keep
        // the handles of every task it ever ran.
        let tasks = Rc::downgrade(self);
        let wrapped = async move {
            task.await;

            if let Some(tasks) = tasks.upgrade() {
                tasks.running.borrow_mut().remove(&id);
            }
        };

        let join = glommio::spawn_local(wrapped).detach();
        self.running.borrow_mut().insert(id, join);

        handle
    }

    fn is_running(&self, id: u64) -> bool {
        self.running.borrow().contains_key(&id)
    }

    fn stop(&self, id: u64) {
        let Some(join) = self.running.borrow_mut().remove(&id) else {
            return;
        };

        join.cancel();
    }

    /// Cancels every task and refuses new ones. A cancelled task is not
    /// polled again; its future is dropped by the executor.
    pub fn close(&self) {
        self.closed.set(true);

        let running = std::mem::take(&mut *self.running.borrow_mut());
        for join in running.into_values() {
            join.cancel();
        }
    }
}

/// Stops a spawned task early. Dropping the handle lets the task run on.
#[derive(Clone)]
pub struct TaskHandle {
    id: u64,
    tasks: Weak<Tasks>,
}

impl TaskHandle {
    /// Cancels the task. It is not polled again, so it stops at the `await`
    /// it is waiting on. Does nothing once the task finished.
    pub fn stop(&self) {
        let Some(tasks) = self.tasks.upgrade() else {
            return;
        };

        tasks.stop(self.id);
    }

    /// Whether the task is still running: not finished, stopped, or ended
    /// with its session.
    pub fn is_running(&self) -> bool {
        let Some(tasks) = self.tasks.upgrade() else {
            return false;
        };

        tasks.is_running(self.id)
    }
}

#[cfg(test)]
#[path = "../tests/tests_tasks.rs"]
mod tests;
