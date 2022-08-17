pub enum ProcessState{
    TaskRunning,
    TaskKillable,
    TaskUninterrutpible,
    TaskInterruptible,
    TaskStopped,
    TaskTraced,
    ExitZombie,
    ExitDead
}

pub struct ProcessID{
    pid: uint
}
