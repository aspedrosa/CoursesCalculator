package results

import (
	. "courses-calculator/storage"

	"archive/zip"
	"bytes"
	"fmt"
	"io"
	"net/http"
	"strings"

	"github.com/PuerkitoBio/goquery"
)

var ORIOASIS_BASE_URL = "https://www.orioasis.pt/oasis"

type stage struct {
	Title    string
	FileLink string
}

func GetEventStages(eventID uint32) ([]stage, error) {
	sb := NewStorageBackend()
	fileID := FileIdentifier{Type: FileTypeHTML, EventID: eventID}

	var htmlContent []byte
	if sb.CheckIfExists(fileID) {
		var err error
		htmlContent, err = sb.Read(fileID)
		if err != nil {
			return nil, fmt.Errorf("failed to read html file: %w", err)
		}
	} else {
		fmt.Printf("No html file for event %d, fetching from orioasis\n", eventID)
		resp, err := http.Get(fmt.Sprintf("%s/results.php?action=view_stages&eventid=%d&lang=en_UK", ORIOASIS_BASE_URL, eventID))
		if err != nil {
			return nil, fmt.Errorf("failed to fetch event: %w", err)
		}
		defer resp.Body.Close()

		if resp.StatusCode != http.StatusOK {
			return nil, fmt.Errorf("failed to get event stages: status %d", resp.StatusCode)
		}

		htmlContent, err = io.ReadAll(resp.Body)
		if err != nil {
			return nil, fmt.Errorf("failed to read response body: %w", err)
		}

		if err := sb.Write(fileID, htmlContent); err != nil {
			return nil, fmt.Errorf("failed to write html file: %w", err)
		}
	}

	doc, err := goquery.NewDocumentFromReader(bytes.NewReader(htmlContent))
	if err != nil {
		return nil, fmt.Errorf("failed to parse html: %w", err)
	}

	var finalStages []stage

	// Get results table
	divContent := doc.Find("div.content").First()
	tbody1 := divContent.Find("tbody").First()
	tbody2 := tbody1.Find("table > tbody").First()
	tbody2_children := tbody2.Children()

	var stagesTr *goquery.Selection
	for i := 2; ; i++ { // some events have some information and links between the buttons and the results table
		stagesTr = tbody2_children.Eq(i)

		if strings.Contains(stagesTr.Text(), "Description") && strings.Contains(stagesTr.Text(), "Uploaded on") {
			break
		}
		if i > 10 {
			return nil, fmt.Errorf("failed to find stages table")
		}
	}

	ignoreRestOftrs := false
	stagesTr.Find("tr").Each(func(i int, s *goquery.Selection) {
		if ignoreRestOftrs {
			return
		}
		if i == 0 { // skip table header
			return
		}

		tds := s.Find("td")

		// stages have a link on the first column. Ignore those that dont
		if tds.Eq(0).Find("a").Length() == 0 {
			ignoreRestOftrs = true
			return
		}

		html, _ := s.Html()
		if strings.Contains(html, "Total por somatório de ") {
			return
		}

		if tds.Length() < 3 {
			return
		}

		title := tds.Eq(0).Find("a").First().Text()
		title = strings.ReplaceAll(title, "\n", "")

		link, exists := tds.Eq(2).Find("a").Last().Attr("href")
		if !exists {
			return
		}

		finalStages = append(finalStages, stage{
			Title:    title,
			FileLink: link,
		})
	})

	return finalStages, nil
}

func DownloadStageResultsZip(eventID, stageID uint32, url string) error {
	sb := NewStorageBackend()
	fileID := FileIdentifier{Type: FileTypeZIP, EventID: eventID, StageID: stageID}

	if sb.CheckIfExists(fileID) {
		return nil
	}

	resp, err := http.Get(url)
	if err != nil {
		return fmt.Errorf("failed to download zip: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("failed to download zip: status %d", resp.StatusCode)
	}

	content, err := io.ReadAll(resp.Body)
	if err != nil {
		return fmt.Errorf("failed to read zip content: %w", err)
	}

	if err := sb.Write(fileID, content); err != nil {
		return fmt.Errorf("failed to write zip file: %w", err)
	}

	fmt.Printf("Downloaded zip file for stage %d of event %d\n", stageID, eventID)
	return nil
}

func ExtractStageResultsZip(eventID, stageID uint32) error {
	sb := NewStorageBackend()
	zipFileID := FileIdentifier{Type: FileTypeZIP, EventID: eventID, StageID: stageID}

	zipData, err := sb.Read(zipFileID)
	if err != nil {
		return fmt.Errorf("failed to read zip file: %w", err)
	}

	zipReader, err := zip.NewReader(bytes.NewReader(zipData), int64(len(zipData)))
	if err != nil {
		return fmt.Errorf("failed to open zip archive: %w", err)
	}

	if len(zipReader.File) == 0 {
		return fmt.Errorf("zip archive is empty")
	}

	f := zipReader.File[0]
	rc, err := f.Open()
	if err != nil {
		return fmt.Errorf("failed to open file in zip: %w", err)
	}
	defer rc.Close()

	content, err := io.ReadAll(rc)
	if err != nil {
		return fmt.Errorf("failed to read file in zip: %w", err)
	}

	csvFileID := FileIdentifier{Type: FileTypeUnzippedCSV, EventID: eventID, StageID: stageID}
	if err := sb.Write(csvFileID, content); err != nil {
		return fmt.Errorf("failed to write csv file: %w", err)
	}

	return nil
}
