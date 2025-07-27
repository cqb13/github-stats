package commands

import (
	"dev/cqb13/gstats/utils"
	"dev/cqb13/gstats/utils/ansi"
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

		if strings.Contains(string(resp), `"message":"Not Found"`) {
			fmt.Println("Failed to find repository")
			return
		}

		var releaseList []release

		err = json.Unmarshal(resp, &releaseList)
		if err != nil {
			fmt.Println(err)
			return
		}

		if len(releaseList) == 0 {
			break
		}

		for _, release := range releaseList {
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
				fmt.Printf("%s%-10d%s%-20s%s\n", ansi.Bold, releaseAssetDownloads, ansi.Reset, release.Name, publishedAt)
			}
			downloadCount += releaseAssetDownloads
		}

		releaseCount += len(releaseList)

		page++
	}

	if releaseCount == 0 {
		fmt.Printf("%s/%s has no releases\n", user, repo)
		return
	}

	fmt.Printf("%s/%s has %s%d%s downloads, across %s%d%s releases\n", user, repo, ansi.Bold, downloadCount, ansi.Reset, ansi.Bold, releaseCount, ansi.Reset)
}
