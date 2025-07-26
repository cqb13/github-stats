package utils

import (
	"fmt"
	"io"
	"net/http"
	"time"
)

func RFC3339StrToPrettyStr(str string) (string, error) {
	time, err := time.Parse(time.RFC3339, str)
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("%d/%d/%d", time.Month(), time.Day(), time.Year()), nil
}

func Get(url string) ([]byte, error) {
	resp, err := http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)

	return body, nil
}
