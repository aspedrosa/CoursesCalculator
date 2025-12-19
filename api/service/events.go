package service

/*
#include <stdlib.h>

extern char* process_stage_results_csv(const char* path);
extern void free_string(char* s);
*/
import "C"
import (
	. "courses-calculator/scrapping/events/results"
	. "courses-calculator/storage"
	"fmt"
	"path/filepath"
	"unsafe"
)

func FetchEvent(eventID uint32) error {
	stages, err := GetEventStages(eventID)
	if err != nil {
		return err
	}

	for i, stage := range stages {
		stageID := uint32(i)
		// fmt.Printf("Processing stage %d: %s\n", stageID, stage.Title)

		if err := DownloadStageResultsZip(eventID, stageID, stage.FileLink); err != nil {
			return fmt.Errorf("failed to download stage zip: %w", err)
		}

		if err := ExtractStageResultsZip(eventID, stageID); err != nil {
			return fmt.Errorf("failed to extract stage zip: %w", err)
		}

		// Process CSV with Rust
		csvFileID := FileIdentifier{Type: FileTypeUnzippedCSV, EventID: eventID, StageID: stageID}

		absPath, err := filepath.Abs(csvFileID.Path())
		if err != nil {
			return fmt.Errorf("failed to get absolute path: %w", err)
		}

		cPath := C.CString(absPath)
		cResult := C.process_stage_results_csv(cPath)
		goResult := C.GoString(cResult)

		C.free(unsafe.Pointer(cPath))
		C.free_string(cResult)

		fmt.Printf("Stage %d: %s\n", stageID, goResult)
	}
	return nil
}
