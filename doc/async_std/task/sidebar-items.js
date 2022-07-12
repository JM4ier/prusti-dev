window.SIDEBAR_ITEMS = {"enum":[["Poll","Indicates whether a value is available or if the current task has been scheduled to receive a wakeup instead."]],"fn":[["block_on","Spawns a task and blocks the current thread on its result."],["current","Returns a handle to the current task."],["sleep","Sleeps for the specified amount of time."],["spawn","Spawns a task."],["spawn_blocking","Spawns a blocking task."],["spawn_local","Spawns a task onto the thread-local executor."],["try_current","Returns a handle to the current task if called within the context of a task created by `block_on`, `spawn`, or `Builder::spawn`, otherwise returns `None`."],["yield_now","Cooperatively gives up a timeslice to the task scheduler."]],"macro":[["ready","Extracts the successful type of a `Poll<T>`."]],"struct":[["AccessError","An error returned by `LocalKey::try_with`."],["Builder","Task builder that configures the settings of a new task."],["Context","The `Context` of an asynchronous task."],["JoinHandle","A handle that awaits the result of a task."],["LocalKey","The key for accessing a task-local value."],["Task","A handle to a task."],["TaskId","A unique identifier for a task."],["Waker","A `Waker` is a handle for waking up a task by notifying its executor that it is ready to be run."]]};