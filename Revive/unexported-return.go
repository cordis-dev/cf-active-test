package revive

// TODO(matloob): Choose better names for Fun, Bloc, Goto, etc.
// TODO(matloob): Write a parser for the Func disassembly. Maybe
// the parser can be used instead of Fun.
import (
	"cmd/compile/internal/types"
	"cmd/internal/obj"
	"cmd/internal/src"
	"fmt"
	"reflect"
	"testing"
)

func Eq(cond, sub, alt string) ctrl {
	return ctrl{BlockAMD64EQ, cond, []string{sub, alt}}
}
