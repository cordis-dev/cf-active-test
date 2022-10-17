package revive

func Test() {
	select {
	case <-c:
		panic("abc")
	default:
		select{}
		break
	}    
}
