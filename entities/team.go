package entities

import (
	"github.com/Steelstone3/F1-Management-Simulator/components"
)

type Team struct {
	name          components.TeamName
	driver        []components.Driver
	season_points components.Points
}
