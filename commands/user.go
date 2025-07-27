package commands

import (
	"dev/cqb13/gstats/utils"
	"dev/cqb13/gstats/utils/ansi"
	"encoding/json"
	"fmt"
	"strings"
)

type ghUser struct {
	Login       string `json:"login"`
	SiteAdmin   bool   `json:"site_admin"`
	Name        string `json:"name"`
	Company     string `json:"company"`
	Blog        string `json:"blog"`
	Location    string `json:"location"`
	Email       string `json:"email"`
	Hireable    bool   `json:"hireable"`
	Bio         string `json:"bio"`
	PublicRepos int    `json:"public_repos"`
	PublicGists int    `json:"public_gists"`
	Followers   int    `json:"followers"`
	Following   int    `json:"following"`
	CreatedAt   string `json:"created_at"`
	UpdatedAt   string `json:"updated_at"`
}

func HandleUserCommand(user string, verbose bool) {
	url := fmt.Sprintf("https://api.github.com/users/%s", user)

	resp, err := utils.Get(url)
	if err != nil {
		fmt.Println(err)
		return
	}

	if strings.Contains(string(resp), `"message":"Not Found"`) {
		fmt.Println("Failed to find user")
		return
	}

	var userData ghUser

	err = json.Unmarshal(resp, &userData)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Printf("%s%s%s%s", ansi.Bold, ansi.Underline, userData.Login, ansi.Reset)
	if userData.Name != "" {
		fmt.Printf(" %saka %s%s\n", ansi.Dim, userData.Name, ansi.Reset)
	} else {
		fmt.Printf("\n")
	}
	if userData.Bio != "" {
		fmt.Printf("%s\n", userData.Bio)
	}
	if userData.Blog != "" {
		if !strings.HasPrefix(userData.Blog, "https://") {
			fmt.Printf("%shttps://", ansi.Dim)
		}
		fmt.Printf("%s%s%s\n", ansi.Dim, userData.Blog, ansi.Reset)
	}

	fmt.Println()
	createdAt, err := utils.RFC3339StrToPrettyStr(userData.CreatedAt)
	if err != nil {
		fmt.Println(err)
		return
	}
	updatedAt, err := utils.RFC3339StrToPrettyStr(userData.UpdatedAt)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Printf("Joined On: %s%-12s%sLast Update: %s%-12s%s\n", ansi.Bold, createdAt, ansi.Reset, ansi.Bold, updatedAt, ansi.Reset)
	fmt.Printf("Public Repos: %s%-10d%sPublic Gists: %s%d%s\n", ansi.Bold, userData.PublicRepos, ansi.Reset, ansi.Bold, userData.PublicGists, ansi.Reset)
	fmt.Printf("Followers: %s%-10d%sFollowing: %s%d%s\n", ansi.Bold, userData.Followers, ansi.Reset, ansi.Bold, userData.Following, ansi.Reset)

	if verbose {
		fmt.Println()
		fmt.Printf("%sHireable: %s", ansi.Reset, ansi.Bold)
		if userData.Hireable {
			fmt.Printf("True\n")
		} else {
			fmt.Printf("False\n")
		}
		fmt.Printf("%sSite Admin: %s", ansi.Reset, ansi.Bold)
		if userData.SiteAdmin {
			fmt.Printf("True\n")
		} else {
			fmt.Printf("False\n")
		}
		fmt.Printf("%s\n", ansi.Reset)
		if userData.Company != "" {
			fmt.Printf("Working At: %s%s%s\n", ansi.Bold, strings.Replace(userData.Company, "@", "", 1), ansi.Reset)
		}
		if userData.Location != "" {
			fmt.Printf("Located In: %s%s%s\n", ansi.Bold, userData.Location, ansi.Reset)
		}
	}

	if userData.Email != "" {
		fmt.Printf("\n%s\n", userData.Email)
	}
}
