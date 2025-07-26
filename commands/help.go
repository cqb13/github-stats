package commands

import (
	"dev/cqb13/gstats/utils/ansi"
	"fmt"
)

func Help() {
	fmt.Printf("%sgstats%s\n\n", ansi.Bold, ansi.Reset)
	fmt.Printf("%scommon args%s\n", ansi.Bold, ansi.Reset)
	fmt.Println("\t--verbose: shows more information")
	fmt.Println()
	fmt.Printf("%scommands%s\n", ansi.Bold, ansi.Reset)
	fmt.Println("\thelp: shows what commands do")
	fmt.Println()
	fmt.Println("\tdownloads: shows download counts of github releases")
	fmt.Println("\t\t[user] [repo]")
	fmt.Println()
}
