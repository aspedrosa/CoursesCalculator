package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func RegisterControllers(r *gin.Engine) {
	r.GET("/health", func(c *gin.Context) {
		c.String(http.StatusOK, "Ok")
	})

	r.POST("/fetch_events", FetchEvents)
}
