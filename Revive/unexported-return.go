package revive

func Eq(cond, sub, alt string) ctrl {
	return ctrl{BlockAMD64EQ, cond, []string{sub, alt}}
}
