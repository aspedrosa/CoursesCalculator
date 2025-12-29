package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func SetupRouter() *gin.Engine {
	r := gin.Default()

	r.GET("/health", func(c *gin.Context) {
		c.String(http.StatusOK, "Ok")
	})

	r.POST("/fetch_events", FetchEvents)

	sessionsGroup := r.Group("/course_calculator_session")
	sessionsGroup.POST("", CreateCourseCalculatorSession)
	sessionsGroup.POST("/:sessionID/attach", AttachEventStageToSession)
	sessionsGroup.POST("/:sessionID/remove", RemoveEventStageFromSession)

	return r
}
