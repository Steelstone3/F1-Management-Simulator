package driver

import (
	"github.com/Steelstone3/F1-Management-Simulator/components/grid"
	"github.com/Steelstone3/F1-Management-Simulator/components/car"
	"github.com/Steelstone3/F1-Management-Simulator/components/names"
)

type Driver struct {
	driverName names.DriverName
	teamName   names.TeamName
	car        car.Car
	statisics  DriverStatistics
	racePoints []grid.Points
}

func constructDriver() Driver {
	return Driver{}
}

func (d *Driver) calculateDriverSeasonPoints() uint {
	return 0
}
