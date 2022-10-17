package revive

func test() {
	ctx := context.WithValue(r.Context(), string("principal_id"), *id)
}
