package main

import (
	"courses-calculator/controllers"
)

func main() {
	r := controllers.SetupRouter()

	err := r.Run(":3000")
	if err != nil {
		panic(err)
	}
}
