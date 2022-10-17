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

// fun is the return type of Fun. It contains the created func
// itself as well as indexes from block and value names into the
// corresponding Blocks and Values.
type fun struct {
	f      *Func
	blocks map[string]*Block
	values map[string]*Value
}

func (c *Conf) Fun(entry string, blocs ...bloc) fun {
	return fun{f, blocks, values}
}
