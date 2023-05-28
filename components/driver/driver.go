package driver

import (
	"github.com/Steelstone3/F1-Management-Simulator/components"
	"github.com/Steelstone3/F1-Management-Simulator/components/car"
)

type Driver struct {
	driverName DriverName
	teamName   components.TeamName
	car        car.Car
	statisics  DriverStatistics
	racePoints []components.Points
}

func constructDriver() Driver {
	return Driver{}
}

func (d *Driver) calculateDriverSeasonPoints() uint {
	return 0
}
