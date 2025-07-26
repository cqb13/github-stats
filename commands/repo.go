package commands

import (
	"dev/cqb13/gstats/utils"
	"encoding/json"
	"fmt"
	"strings"
)

type repository struct {
	Description   string   `json:"description"`
	Fork          bool     `json:"fork"`
	CreatedAt     string   `json:"created_at"`
	UpdatedAt     string   `json:"updated_at"`
	PushedAt      string   `json:"pushed_at"`
	Homepage      string   `json:"homepage"`
	Stars         int      `json:"stargazers_count"`
	Language      string   `json:"language"`
	Forks         int      `json:"forks_count"`
	Archived      bool     `json:"archived"`
	OpenIssues    int      `json:"open_issues_count"`
	IsTemplate    bool     `json:"is_template"`
	Topics        []string `json:"topics"`
	DefaultBranch string   `json:"default_branch"`
}

func HandleRepoCommand(user string, repo string, verbose bool) {
	url := fmt.Sprintf("https://api.github.com/repos/%s/%s", user, repo)

	resp, err := utils.Get(url)
	if err != nil {
		fmt.Println(err)
		return
	}

	var repositoryData repository

	if strings.Contains(string(resp), `"message":"Not Found"`) {
		fmt.Println("Failed to find repository")
		return
	}

	err = json.Unmarshal(resp, &repositoryData)
	if err != nil {
		fmt.Println(err)
		return
	}
}
