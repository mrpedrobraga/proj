//! # Task
//!
//! A [Future] with super powers.
//!
//! Tasks are asynchronous, composable, and, most of all, inspectable.

use std::{pin::Pin, task::Context};

pub mod primitives;

/// See the [module-level documentation][self].
pub trait Task {
    type Output;
    type StepIter: Iterator<Item = TaskStep>;

    /// Polls the task, in a similar fashion to [Future],
    /// except that a Task has more information available in its pending state.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;

    /// Returns information about the task.
    fn info(&self) -> TaskInfo;

    fn steps(&self) -> Self::StepIter;
}

pub enum Poll<T> {
    Pending(TaskProgressState),
    Done(T),
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    /// The amount of steps this task has.
    pub progress_step_count: Option<usize>,

    /// A label for this task, to make it easily identifiable.
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TaskProgressState {
    /// How many steps this task has progressed so far.
    /// Combined with [TaskInfo::progress_step_count], can be used to make a progress bar.
    pub progress_step: usize,
    /// What the task is currently doing.
    pub progress_label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TaskStep {
    pub label: Option<String>
}
