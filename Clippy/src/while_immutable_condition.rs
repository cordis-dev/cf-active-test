pub fn wait(ready: bool) -> u32 {
    let mut ticks = 0;
    while !ready {
        ticks += 1;
    }
    ticks
}
