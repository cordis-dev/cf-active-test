pub async fn compute() -> i32 {
    42
}

pub fn start() -> impl Future {
    async { compute() }
}
