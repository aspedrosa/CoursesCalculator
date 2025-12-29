package session

import (
	"courses-calculator/models"
	"courses-calculator/storage"
	"errors"
)

func CreateSession(name, description string) uint {
	// TODO unique name
	session := models.CourseCalculatorSession{
		Name:        name,
		Description: description,
	}
	storage.GetDB().Create(&session)
	return session.ID
}

func AttachEventStageToSession(sessionID uint, eventID, stageNumber uint) error {
	// TODO check if eventID+stageID make sense on the orioasis world
	db := storage.GetDB()
	var session models.CourseCalculatorSession
	db.Preload("Stages").First(&session, sessionID)
	if session.ID == 0 {
		return errors.New("session not found")
	}

	// Ensure the event exists. If not, create it with the provided ID.
	var event models.Event
	if err := db.FirstOrCreate(&event, models.Event{ID: eventID}).Error; err != nil {
		return err
	}

	eventStage := models.EventStage{EventID: eventID, StageNumber: stageNumber}
	if err := db.FirstOrCreate(&eventStage, eventStage).Error; err != nil {
		return err
	}

	for _, stage := range session.Stages {
		if stage.EventID == eventID && stage.StageNumber == stageNumber {
			return nil
		}
	}

	err := db.Model(&session).Association("Stages").Append(eventStage)
	if err != nil {
		return errors.New("unable to add stage for session")
	}

	return nil
}

func RemoveStageFromSession(sessionID uint, eventID, stageNumber uint) error {
	db := storage.GetDB()
	var session models.CourseCalculatorSession
	db.Preload("Stages").First(&session, sessionID)
	if session.ID == 0 {
		return errors.New("session not found")
	}

	foundIdx := -1
	for i, stage := range session.Stages {
		if stage.EventID == eventID && stage.StageNumber == stageNumber {
			foundIdx = i
			break
		}
	}

	if foundIdx == -1 {
		return errors.New("stage not found in session")
	}

	err := db.Model(&session).Association("Stages").Delete(session.Stages[foundIdx])
	if err != nil {
		return errors.New("unable to replace stages for session")
	}
	db.Save(&session)

	return nil
}

func ImportStagesDataOfSession() {

}

func GetAveragePacesOfSession() {

}

func GetEstimatedTimesForSession(sessionID uint, class string) {
}
