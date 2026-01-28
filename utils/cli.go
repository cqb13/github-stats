package utils

import "strings"

type FlagSet struct {
	boolflags map[string]*BoolFlag
	posargs   []string
}

type BoolFlag struct {
	name         string
	defaultvalue bool
	found        bool
}

func NewFlagSet() *FlagSet {
	return &FlagSet{
		boolflags: make(map[string]*BoolFlag),
	}
}

func (fs *FlagSet) AddBoolFlag(name string, defaultvalue bool) {
	fs.boolflags[name] = &BoolFlag{
		name:         name,
		defaultvalue: defaultvalue,
		found:        false,
	}
}

func (fs *FlagSet) Parse(args []string) {
	for _, arg := range args {
		if after, ok := strings.CutPrefix(arg, "--"); ok {
			name := after
			if flag, exists := fs.boolflags[name]; exists {
				flag.found = true
			}
		} else {
			fs.posargs = append(fs.posargs, arg)
		}
	}
}

func (fs *FlagSet) GetBool(name string) bool {
	if flag, exists := fs.boolflags[name]; exists {
		return flag.found || flag.defaultvalue
	}
	return false
}

func (fs *FlagSet) PosArgs() []string {
	return fs.posargs
}
