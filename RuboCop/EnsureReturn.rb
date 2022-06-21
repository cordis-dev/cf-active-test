# bad

def foo
  do_something
ensure
  cleanup
  return self
end
