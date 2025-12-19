package storage

import (
	"fmt"
	"os"
)

type LocalBackend struct{}

func NewStorageBackend() StorageBackend {
	backend := os.Getenv("STORAGE_BACKEND")
	if backend == "s3" {
		// return &S3Backend{} // Not implemented yet
		return &LocalBackend{}
	}
	return &LocalBackend{}
}

func (l *LocalBackend) CheckIfExists(fileID FileIdentifier) bool {
	_, err := os.Stat(fileID.Path())
	return !os.IsNotExist(err)
}

func (l *LocalBackend) Read(fileID FileIdentifier) ([]byte, error) {
	return os.ReadFile(fileID.Path())
}

func (l *LocalBackend) Write(fileID FileIdentifier, data []byte) error {
	dir := fileID.ParentDir()
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory %s: %w", dir, err)
	}
	return os.WriteFile(fileID.Path(), data, 0644)
}
