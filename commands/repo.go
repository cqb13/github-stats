package commands

import (
	"dev/cqb13/gstats/utils"
	"dev/cqb13/gstats/utils/ansi"
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

	var rd repository

	if strings.Contains(string(resp), `"message":"Not Found"`) {
		fmt.Println("Failed to find repository")
		return
	}

	err = json.Unmarshal(resp, &rd)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Printf("%s%s%s%s\n", ansi.Bold, ansi.Underline, repo, ansi.Reset)
	fmt.Println(rd.Description)
	if verbose && rd.Homepage != "" {
		fmt.Printf("%s%s%s\n", ansi.Dim, rd.Homepage, ansi.Reset)
	}
	fmt.Println()
	createdAt, err := utils.RFC3339StrToPrettyStr(rd.CreatedAt)
	if err != nil {
		fmt.Println(err)
		return
	}
	pushedAt, err := utils.RFC3339StrToPrettyStr(rd.PushedAt)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Printf("Created On: %s%-12s%sLast Push: %s%-12s%s", ansi.Bold, createdAt, ansi.Reset, ansi.Bold, pushedAt, ansi.Reset)
	if verbose {
		updatedAt, err := utils.RFC3339StrToPrettyStr(rd.UpdatedAt)
		if err != nil {
			fmt.Println(err)
			return
		}
		fmt.Printf("Last Update: %s%-12s%s\n", ansi.Bold, updatedAt, ansi.Reset)
	} else {
		fmt.Printf("\n")
	}
	fmt.Printf("Stars: %s%-10d%s", ansi.Bold, rd.Stars, ansi.Reset)
	if verbose {
		fmt.Printf("Forks: %s%-10d%sOpen Issues: %s%d%s\n", ansi.Bold, rd.Forks, ansi.Reset, ansi.Bold, rd.OpenIssues, ansi.Reset)
	} else {
		fmt.Printf("\n")
	}
	fmt.Printf("Default Branch: %s%s%-5sMain Language: %s%s%s\n", ansi.Bold, rd.DefaultBranch, ansi.Reset, ansi.Bold, rd.Language, ansi.Reset)
	fmt.Println()
	fmt.Printf("Fork: %s", ansi.Bold)
	if rd.Fork {
		fmt.Printf("True\n")
	} else {
		fmt.Printf("False\n")
	}
	if verbose {
		fmt.Printf("%sArchived: %s", ansi.Reset, ansi.Bold)
		if rd.Archived {
			fmt.Printf("True\n")
		} else {
			fmt.Printf("False\n")
		}
		fmt.Printf("%sTemplate: %s", ansi.Reset, ansi.Bold)
		if rd.IsTemplate {
			fmt.Printf("True\n")
		} else {
			fmt.Printf("False\n")
		}
	}
	fmt.Printf("%s", ansi.Reset)

	if verbose {
		fmt.Println()
		fmt.Println(createTopicStr(rd.Topics))
	}
}

func createTopicStr(topics []string) string {
	topicStr := ""
	perLine := 3

	length := len(topics) - 1

	for i, topic := range topics {
		topicStr += fmt.Sprintf("%s%s%s", ansi.Bold, topic, ansi.Reset)
		if i != length {
			topicStr += ", "
			if i%perLine == 0 && i >= perLine {
				topicStr += "\n"
			}
		}
	}

	return topicStr
}
