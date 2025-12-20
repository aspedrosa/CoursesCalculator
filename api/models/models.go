package models

import (
	"gorm.io/gorm"
)

type Event struct {
	gorm.Model
	ID   uint `gorm:"primaryKey"`
	Name string
}

type EventStage struct {
	gorm.Model
	EventID         uint `gorm:"uniqueIndex:idx_event_stage"`
	StageNumber     uint `gorm:"uniqueIndex:idx_event_stage"`
	Event           Event
	Name            string
	ResultsImported bool `gorm:"default:false"`
}

type CourseCalculatorSession struct {
	gorm.Model
	ID          uint `gorm:"primaryKey"`
	Name        string
	Description string
	Stages      []EventStage `gorm:"many2many:session_stages;"`
}
