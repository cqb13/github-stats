package commands

import (
	"dev/cqb13/gstats/utils"
	"encoding/json"
	"fmt"
	"strings"
)

type release struct {
	Name        string `json:"name"`
	PublishedAt string `json:"published_at"`
	Assets      []struct {
		Downloads int `json:"download_count"`
	} `json:"assets"`
}

func HandleDownloadsCommand(user string, repo string, verbose bool) {
	baseUrl := fmt.Sprintf("https://api.github.com/repos/%s/%s/releases?per_page=100&page=", user, repo)

	downloadCount := 0
	releaseCount := 0
	page := 1

	for {
		resp, err := utils.Get(fmt.Sprintf("%s%d", baseUrl, page))
		if err != nil {
			fmt.Println(err)
			return
		}

		var releases []release

		if strings.Contains(string(resp), `"message":"Not Found"`) {
			fmt.Println("Failed to find repository")
			return
		}

		err = json.Unmarshal(resp, &releases)
		if err != nil {
			fmt.Println(err)
			return
		}

		if len(releases) == 0 {
			break
		}

		for _, release := range releases {
			releaseAssetDownloads := 0
			for _, asset := range release.Assets {
				releaseAssetDownloads += asset.Downloads
			}
			if verbose {
				publishedAt, err := utils.RFC3339StrToPrettyStr(release.PublishedAt)
				if err != nil {
					fmt.Println(err)
					return
				}
				fmt.Printf("%-10d%-20s%s\n", releaseAssetDownloads, release.Name, publishedAt)
			}
			downloadCount += releaseAssetDownloads
		}

		releaseCount += len(releases)

		page++
	}

	if releaseCount == 0 {
		fmt.Printf("%s/%s has no releases\n", user, repo)
		return
	}

	fmt.Printf("%s/%s has %d downloads, across %d releases\n", user, repo, downloadCount, releaseCount)
}
