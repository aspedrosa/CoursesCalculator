package controllers

import (
	service "courses-calculator/service/course_calculator_session"
	"strconv"

	"github.com/gin-gonic/gin"
)

type CourseCalculatorSession struct {
	Name        string
	Description string
}

type EventStage struct {
	EventID     uint
	StageNumber uint
}

func CreateCourseCalculatorSession(c *gin.Context) {
	var session CourseCalculatorSession
	err := c.ShouldBindJSON(&session)
	if err != nil {
		c.JSON(400, gin.H{"error": err.Error()})
		return
	}
	service.CreateSession(session.Name, session.Description)
	c.JSON(200, session)
}

func AttachEventStageToSession(c *gin.Context) {
	var eventStage EventStage
	err := c.ShouldBindJSON(&eventStage)
	if err != nil {
		c.JSON(400, gin.H{"error": err.Error()})
		return
	}
	sessionIDParam := c.Param("sessionID")

	sessionID, err := strconv.ParseUint(sessionIDParam, 10, 32)
	if err != nil {
		c.JSON(400, gin.H{"error": "Invalid session ID"})
		return
	}

	err = service.AttachEventStageToSession(uint(sessionID), eventStage.EventID, eventStage.StageNumber)
	if err != nil {
		c.JSON(400, gin.H{"error": err.Error()})
		return
	}
	c.JSON(200, eventStage)
}

func RemoveEventStageFromSession(c *gin.Context) {
	var eventStage EventStage
	err := c.ShouldBindJSON(&eventStage)
	if err != nil {
		c.JSON(400, gin.H{"error": err.Error()})
		return
	}
	sessionIDParam := c.Param("sessionID")

	sessionID, err := strconv.ParseUint(sessionIDParam, 10, 32)
	if err != nil {
		c.JSON(400, gin.H{"error": "Invalid session ID"})
		return
	}

	err = service.RemoveStageFromSession(uint(sessionID), eventStage.EventID, eventStage.StageNumber)
	if err != nil {
		c.JSON(400, gin.H{"error": err.Error()})
		return
	}
	c.JSON(200, eventStage)
}
