package commands

import "fmt"

func Help() {
	fmt.Println("gstats")
	fmt.Println()
	fmt.Println("COMMON ARGS:")
	fmt.Println("\t--verbose: shows more information")
	fmt.Println()
	fmt.Println("COMMANDS:")
	fmt.Println("\thelp: shows what commands do")
	fmt.Println()
	fmt.Println("\tdownloads: shows download counts of github releases")
	fmt.Println("\t\t[user] [repo]")
	fmt.Println()
}
