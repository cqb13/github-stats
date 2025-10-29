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
	Downloads Command = iota
	Help
	Followers
	Following
	Repo
	Starred
	User
)

func main() {
	args := os.Args[1:]
	if len(args) == 0 {
		commands.Help()
		return
	}
	flags := utils.NewFlagSet()
	flags.AddBoolFlag("verbose", false)

	flags.Parse(args)

	posArgs := flags.PosArgs()

	command := matchCommand(posArgs[0])

	verbose := flags.GetBool("verbose")

	switch command {
	case Downloads:
		if len(args) < 3 {
			fmt.Println("Not enough args: downloads [user] [repo]")
			return
		}

		user := posArgs[1]
		repo := posArgs[2]

		commands.HandleDownloadsCommand(user, repo, verbose)
		return
	case Help:
		commands.Help()
		return
	case Followers:
		if len(args) < 2 {
			fmt.Println("Not enough args: followers [user]")
			return
		}

		user := posArgs[1]

		commands.HandleRelationsCommand(user, commands.Followers, verbose)
		return
	case Following:
		if len(args) < 2 {
			fmt.Println("Not enough args: following [user]")
			return
		}

		user := posArgs[1]

		commands.HandleRelationsCommand(user, commands.Following, verbose)
		return
	case Repo:
		if len(args) < 3 {
			fmt.Println("Not enough args: repo [user] [repo]")
			return
		}

		user := posArgs[1]
		repo := posArgs[2]

		commands.HandleRepoCommand(user, repo, verbose)
		return
	case Starred:
		if len(args) < 2 {
			fmt.Println("Not enough args: user [user]")
			return
		}

		user := posArgs[1]

		commands.HandleStarredCommand(user, verbose)
		return
	case User:
		if len(args) < 2 {
			fmt.Println("Not enough args: user [user]")
			return
		}

		user := posArgs[1]

		commands.HandleUserCommand(user, verbose)
		return
	default:
		fmt.Printf("%s is not a valid command\n", args[0])
		commands.Help()
		return
	}
}

func matchCommand(cmd string) Command {
	switch strings.ToLower(cmd) {
	case "downloads":
		return Downloads
	case "help":
		return Help
	case "followers":
		return Followers
	case "following":
		return Following
	case "repo":
		return Repo
	case "starred":
		return Starred
	case "user":
		return User
	default:
		return Help
	}
}
