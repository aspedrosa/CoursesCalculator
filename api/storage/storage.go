package storage

import (
	"fmt"
	"os"
	"path/filepath"
)

type FileType int

const (
	FileTypeHTML FileType = iota
	FileTypeZIP
	FileTypeUnzippedCSV
)

type FileIdentifier struct {
	Type    FileType
	EventID uint32
	StageID uint32
}

func (f FileIdentifier) Path() string {
	dataRoot := os.Getenv("DATA_ROOT")
	if dataRoot == "" {
		dataRoot = "data" // the default data root is a directory called data on the cwd
	}

	switch f.Type {
	case FileTypeHTML:
		return filepath.Join(dataRoot, "html", fmt.Sprintf("%d.html", f.EventID))
	case FileTypeZIP:
		return filepath.Join(dataRoot, "zip", fmt.Sprintf("%d", f.EventID), fmt.Sprintf("%d.zip", f.StageID))
	case FileTypeUnzippedCSV:
		return filepath.Join(dataRoot, "unzipped", fmt.Sprintf("%d", f.EventID), fmt.Sprintf("%d.csv", f.StageID))
	}
	return ""
}

func (f FileIdentifier) ParentDir() string {
	dataRoot := os.Getenv("DATA_ROOT")
	if dataRoot == "" {
		dataRoot = "data"
	}

	switch f.Type {
	case FileTypeHTML:
		return filepath.Join(dataRoot, "html")
	case FileTypeZIP:
		return filepath.Join(dataRoot, "zip", fmt.Sprintf("%d", f.EventID))
	case FileTypeUnzippedCSV:
		return filepath.Join(dataRoot, "unzipped", fmt.Sprintf("%d", f.EventID))
	}
	return ""
}

type StorageBackend interface {
	CheckIfExists(fileID FileIdentifier) bool
	Read(fileID FileIdentifier) ([]byte, error)
	Write(fileID FileIdentifier, data []byte) error
}
