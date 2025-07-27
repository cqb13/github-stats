package commands

import (
	"dev/cqb13/gstats/utils"
	"dev/cqb13/gstats/utils/ansi"
	"encoding/json"
	"fmt"
	"strings"
)

type RelationType int

const (
	Followers RelationType = iota
	Following
)

func (r RelationType) ToString() string {
	switch r {
	case Followers:
		return "followers"
	case Following:
		return "following"
	default:
		return "unknown"
	}
}

type ghUser struct {
	Login string `json:"login"`
}

func HandleRelationsCommand(user string, relationType RelationType, verbose bool) {
	if relationType.ToString() == "unknown" {
		fmt.Println("Unknown RelationType")
		return
	}

	baseUrl := fmt.Sprintf("https://api.github.com/users/%s/%s?per_page=100&page=", user, relationType.ToString())

	page := 1
	count := 0

	for {
		resp, err := utils.Get(fmt.Sprintf("%s%d", baseUrl, page))
		if err != nil {
			fmt.Println(err)
			return
		}

		if strings.Contains(string(resp), `"message":"Not Found"`) {
			fmt.Println("Failed to find repository")
			return
		}

		var userList []ghUser

		err = json.Unmarshal(resp, &userList)
		if err != nil {
			fmt.Println(err)
			return
		}

		if verbose {
			for i, user := range userList {
				fmt.Printf("%d. %s%s%s\n", i+1+count, ansi.Bold, user.Login, ansi.Reset)
			}
		}

		if len(userList) == 0 {
			break
		}

		count += len(userList)
		page++
	}

	if relationType == Followers {
		fmt.Printf("%s has %s%d%s followers\n", user, ansi.Bold, count, ansi.Reset)
	} else {
		fmt.Printf("%s is following %s%d%s users\n", user, ansi.Bold, count, ansi.Reset)
	}
}
