use std::time::Duration;

use glommio::LocalExecutorBuilder;
use glommio::timer::sleep;

use super::*;

/// A task that counts once now and once after a short sleep.
fn counter(count: &Rc<Cell<u32>>) -> impl Future<Output = ()> + 'static {
    let count = count.clone();

    async move {
        count.set(count.get() + 1);
        sleep(Duration::from_millis(5)).await;
        count.set(count.get() + 1);
    }
}

#[test]
fn finished_tasks_forget_themselves_and_stopped_ones_never_resume() {
    LocalExecutorBuilder::default().make().unwrap().run(async {
        let tasks = Rc::new(Tasks::default());
        let finished = Rc::new(Cell::new(0));
        let stopped = Rc::new(Cell::new(0));

        let finishing = tasks.spawn(counter(&finished));
        let stopping = tasks.spawn(counter(&stopped));
        sleep(Duration::from_millis(1)).await;

        stopping.stop();
        assert!(!stopping.is_running());
        assert!(finishing.is_running());

        sleep(Duration::from_millis(20)).await;
        assert_eq!(finished.get(), 2);
        assert_eq!(stopped.get(), 1);
        assert!(!finishing.is_running());
        assert!(tasks.running.borrow().is_empty());
    });
}

#[test]
fn closing_cancels_every_task_and_refuses_new_ones() {
    LocalExecutorBuilder::default().make().unwrap().run(async {
        let tasks = Rc::new(Tasks::default());
        let count = Rc::new(Cell::new(0));

        tasks.spawn(counter(&count));
        sleep(Duration::from_millis(1)).await;
        tasks.close();

        let late = tasks.spawn(counter(&count));
        assert!(!late.is_running());

        sleep(Duration::from_millis(20)).await;
        assert_eq!(count.get(), 1);
    });
}
