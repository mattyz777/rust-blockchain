

pub fn caller() -> anyhow::Result<String> {
    worker()
}

pub fn worker() -> anyhow::Result<String> {
    Err(anyhow::anyhow!("worker error"))
}