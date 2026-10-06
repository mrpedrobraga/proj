use crate::task::Task;

use super::{Chain, TaskFuture};

pub trait TaskExt: Task {
    fn chain<Other: Task>(self, other: Other) -> Chain<Self, Other>
    where
        Self: std::marker::Sized,
    {
        Chain { a: self, b: other, _a_is_done: false }
    }

    fn into_future(self) -> TaskFuture<Self> where Self: Sized {
        TaskFuture { task: self }
    }
}

impl<T> TaskExt for T where T: Task {}