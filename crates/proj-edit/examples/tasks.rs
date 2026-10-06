use ::futures_time::task::sleep;
use ::proj_edit::task::{Task, primitives::task_ext::TaskExt as _};
use futures_time::{time::Duration};
use proj_edit::task::primitives::FutureTask;

fn main() {
    let task_chain = FutureTask::new(
        async {
            sleep(Duration::from_millis(1000)).await;
        },
        Some("Fetching value".into()),
    )
    .chain(FutureTask::new(
        async {
            sleep(Duration::from_millis(1000)).await;
        },
        Some("Computing other stuff".into()),
    ))
    .chain(FutureTask::new(
        async {
            sleep(Duration::from_millis(1000)).await;
            42
        },
        Some("Finalising...".into()),
    ));

    dbg!(task_chain.steps().collect::<Vec<_>>());

    let result = futures::executor::block_on(task_chain.into_future());
    dbg!(result);
}
