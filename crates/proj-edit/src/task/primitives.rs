use pin_project::pin_project;

use super::{Poll, Task, TaskInfo, TaskPendingState};

#[pin_project]
/// Adapts (upgrades) a simple Future to a Task!
pub struct FutureTask<F> {
    #[pin]
    fut: F,
    info: TaskInfo,
}

impl<F> FutureTask<F> {
    /// Creates a new FutureTask from a future, adding to it some information.
    pub fn new(fut: F, info: TaskInfo) -> Self {
        Self { fut, info }
    }
}

impl<F> Task for FutureTask<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> super::Poll<Self::Output> {
        let this = self.project();

        match this.fut.poll(cx) {
            std::task::Poll::Ready(result) => Poll::Done(result),
            std::task::Poll::Pending => Poll::Pending(TaskPendingState { progress_step: 0 }),
        }
    }

    fn info(&self) -> super::TaskInfo {
        self.info.clone()
    }
}

#[pin_project]
/// Adapts (downgrades) a Task to a Future.
pub struct TaskFuture<T> {
    #[pin]
    task: T,
}

impl<T> TaskFuture<T> {
    pub fn new(task: T) -> Self {
        Self { task }
    }
}

impl<T> Future for TaskFuture<T>
where
    T: Task,
{
    type Output = T::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();

        match this.task.poll(cx) {
            Poll::Pending(_task_pending_state) => std::task::Poll::Pending,
            Poll::Done(result) => std::task::Poll::Ready(result),
        }
    }
}
