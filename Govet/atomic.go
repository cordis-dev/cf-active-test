func AtomicTest() {
	x := uint64(1)
	x = atomic.AddUint64(&x, 1) // direct assignment to atomic value
}
