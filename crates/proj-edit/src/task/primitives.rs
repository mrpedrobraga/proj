use ::std::iter::Once;

use super::{Poll, Task, TaskInfo, TaskProgressState, TaskStep};
use pin_project::pin_project;

pub mod task_ext;

#[pin_project]
/// Adapts (upgrades) a simple Future to a Task!
pub struct FutureTask<F> {
    #[pin]
    fut: F,
    info: TaskInfo,
}

impl<F> FutureTask<F> {
    /// Creates a new FutureTask from a future, adding to it some information.
    pub fn new(fut: F, label: Option<String>) -> Self {
        Self {
            fut,
            info: TaskInfo {
                progress_step_count: Some(1),
                label,
            },
        }
    }
}

impl<F> Task for FutureTask<F>
where
    F: Future,
{
    type Output = F::Output;
    type StepIter = Once<TaskStep>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> super::Poll<Self::Output> {
        let this = self.project();

        match this.fut.poll(cx) {
            std::task::Poll::Ready(result) => Poll::Done(result),
            std::task::Poll::Pending => Poll::Pending(TaskProgressState {
                progress_step: 0,
                progress_label: this.info.label.clone(),
            }),
        }
    }

    fn info(&self) -> super::TaskInfo {
        self.info.clone()
    }

    fn steps(&self) -> Self::StepIter {
        std::iter::once(TaskStep {
            label: self.info.label.clone(),
        })
    }
}

#[pin_project::pin_project]
pub struct Chain<A, B> {
    #[pin]
    a: A,
    #[pin]
    b: B,
    _a_is_done: bool,
}

impl<A, B> Task for Chain<A, B>
where
    A: Task<Output = ()>,
    B: Task,
{
    type Output = B::Output;
    type StepIter = std::iter::Chain<A::StepIter, B::StepIter>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let a_info = this.a.info();

        if !*this._a_is_done {
            match this.a.poll(cx) {
                Poll::Pending(task_pending_state) => {
                    return Poll::Pending(TaskProgressState {
                        ..task_pending_state
                    });
                }
                Poll::Done(_) => {
                    *this._a_is_done = true;
                }
            };
        }

        match this.b.poll(cx) {
            Poll::Pending(task_pending_state) => Poll::Pending(TaskProgressState {
                progress_step: a_info.progress_step_count.unwrap_or(0)
                    + task_pending_state.progress_step,
                ..task_pending_state
            }),
            Poll::Done(result) => Poll::Done(result),
        }
    }

    fn info(&self) -> TaskInfo {
        let a_info = self.a.info();
        let b_info = self.b.info();

        TaskInfo {
            progress_step_count: a_info
                .progress_step_count
                .zip(b_info.progress_step_count)
                .map(|(a, b)| a + b),
            label: None,
        }
    }

    fn steps(&self) -> Self::StepIter {
        self.a.steps().chain(self.b.steps())
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
        let task_info = this.task.info();
        match this.task.poll(cx) {
            Poll::Pending(_task_pending_state) => {
                let progress_percentage = (_task_pending_state.progress_step as f32
                    / task_info.progress_step_count.unwrap_or_default() as f32)
                    * 100.0;
                let label = _task_pending_state
                    .progress_label
                    .unwrap_or_else(|| "Unknown...".to_string());

                println!("'{label}' ({progress_percentage}%)",);
                std::task::Poll::Pending
            }
            Poll::Done(result) => {
                println!("Done.");
                std::task::Poll::Ready(result)
            }
        }
    }
}
