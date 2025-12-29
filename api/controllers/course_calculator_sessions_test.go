package controllers

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/go-playground/assert/v2"
)

func TestAttachEventStageToSession_EmptySessionID(t *testing.T) {
	router := SetupRouter()
	w := httptest.NewRecorder()

	payload := EventStage{
		EventID:     1,
		StageNumber: 1,
	}
	body, _ := json.Marshal(payload)

	req, _ := http.NewRequest("POST", "/course_calculator_session//attach", bytes.NewReader(body))
	router.ServeHTTP(w, req)

	assert.Equal(t, 400, w.Code)
	assert.Equal(t, "{\"error\":\"Invalid session ID\"}", w.Body.String())
}

func TestAttachEventStageToSession_InvalidNonNumericSessionID(t *testing.T) {
	router := SetupRouter()
	w := httptest.NewRecorder()

	payload := EventStage{
		EventID:     1,
		StageNumber: 1,
	}
	body, _ := json.Marshal(payload)

	req, _ := http.NewRequest("POST", "/course_calculator_session/sss/attach", bytes.NewReader(body))
	router.ServeHTTP(w, req)

	assert.Equal(t, 400, w.Code)
	assert.Equal(t, "{\"error\":\"Invalid session ID\"}", w.Body.String())
}
