package commands

import (
	"dev/cqb13/gstats/utils"
	"dev/cqb13/gstats/utils/ansi"
	"encoding/json"
	"fmt"
	"slices"
	"strings"
)

type repo struct {
	Name  string `json:"name"`
	Owner struct {
		Login string `json:"login"`
	} `json:"owner"`
	URL string `json:"html_url"`
}

func HandleStarredCommand(user string, verbose bool) {
	baseUrl := fmt.Sprintf("https://api.github.com/users/%s/starred?per_page=100&page=", user)

	page := 1
	var repos []repo

	for {
		resp, err := utils.Get(fmt.Sprintf("%s%d", baseUrl, page))
		if err != nil {
			fmt.Println(err)
			return
		}

		if strings.Contains(string(resp), `"message":"Not Found"`) {
			fmt.Println("Failed to find user")
			return
		}

		var repoList []repo

		err = json.Unmarshal(resp, &repoList)
		if err != nil {
			fmt.Println(err)
			return
		}

		if len(repoList) == 0 {
			break
		}

		repos = slices.Concat(repos, repoList)

		page++
	}

	if verbose {
		for i, repo := range repos {
			fmt.Printf("%d. %s/%s %s(%s)%s\n", i+1, repo.Owner.Login, repo.Name, ansi.Dim, repo.URL, ansi.Reset)
		}
	}
	fmt.Printf("%s has starred %d repositories\n", user, len(repos))
}
