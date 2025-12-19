package main

import (
	"courses-calculator/controllers"

	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.Default()

	controllers.RegisterControllers(r)

	r.Run(":3000")
}
