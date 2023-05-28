package entities

import (
	"github.com/Steelstone3/F1-Management-Simulator/components"
	"github.com/Steelstone3/F1-Management-Simulator/components/car"
	"github.com/Steelstone3/F1-Management-Simulator/components/driver"
)

type Team struct {
	name       components.TeamName
	car        car.Car
	driver     []driver.Driver
	racePoints []components.Points
}

func constructTeam() Team {
	return Team{}
}

func (t *Team) calculateTeamSeasonPoints() uint {
	return 0
}
