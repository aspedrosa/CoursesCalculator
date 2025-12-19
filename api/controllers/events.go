package controllers

import (
	"courses-calculator/service"
	"fmt"
	"net/http"
	"sync"

	"github.com/gin-gonic/gin"
)

func FetchEvents(c *gin.Context) {
	var eventIDs []uint32
	if err := c.ShouldBindJSON(&eventIDs); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	fmt.Printf("Received request for events: %v\n", eventIDs)

	var wg sync.WaitGroup
	results := make([]string, len(eventIDs))

	for i, id := range eventIDs {
		wg.Add(1)
		go func(i int, id uint32) {
			defer wg.Done()
			if err := service.FetchEvent(id); err != nil {
				results[i] = fmt.Sprintf("Error: %v", err)
			} else {
				results[i] = "Success"
			}
		}(i, id)
	}

	wg.Wait()

	c.JSON(http.StatusOK, results)
}
