package main

import (
	"dev/cqb13/gstats/commands"
	"dev/cqb13/gstats/utils"
	"fmt"
	"os"
	"strings"
)

type Command int

const (
	Downloads Command = 0
	Help      Command = 1
)

func main() {
	args := os.Args[1:]

	flags := utils.NewFlagSet()
	flags.AddBoolFlag("verbose", false)

	flags.Parse(args)

	posArgs := flags.PosArgs()

	command, err := matchCommand(posArgs[0])
	if err != nil {
		fmt.Println(err)
		return
	}

	switch command {
	case Downloads:
		if len(args) < 3 {
			fmt.Println("Not enough args: downloads user repo")
			return
		}

		user := posArgs[1]
		repo := posArgs[2]
		verbose := flags.GetBool("verbose")

		commands.HandleDownloadsCommand(user, repo, verbose)
		return
	case Help:
		commands.Help()
		return
	default:
		fmt.Printf("%s is not a valid command\n", args[0])
		return
	}
}

func matchCommand(cmd string) (Command, error) {
	switch strings.ToLower(cmd) {
	case "downloads":
		return Downloads, nil
	case "help":
		return Help, nil
	default:
		return -1, fmt.Errorf("%s is not a valid command", cmd)
	}
}
