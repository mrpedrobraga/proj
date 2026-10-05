use futures_time::{future::FutureExt, time::Duration};
use proj_edit::task::{
    primitives::{FutureTask, TaskFuture},
    TaskInfo,
};

fn main() {
    println!("Starting!");

    let fut = async { "This is my value." }.delay(Duration::from_millis(1000));
    let futtask = FutureTask::new(
        fut,
        TaskInfo {
            progress_step_count: None,
            label: Some("Fetching Value".to_string()),
        },
    );
    let futtaskfut = TaskFuture::new(futtask);

    let result = futures::executor::block_on(futtaskfut);
    dbg!(result);
}
