use edit::task::{
    primitives::{FutureTask, TaskFuture},
    TaskInfo,
};
use futures_time::{future::FutureExt, time::Duration};

fn main() {
    println!("Starting!");

    let fut = async { "This is my value." }.delay(Duration::from_millis(1000));
    let tt = FutureTask::new(
        fut,
        TaskInfo {
            progress_step_count: None,
            label: Some("Fetching Value".to_string()),
        },
    );
    let ttfut = TaskFuture::new(tt);

    let result = futures::executor::block_on(ttfut);
    dbg!(result);
}
