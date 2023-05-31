package entities

import (
	"github.com/Steelstone3/F1-Management-Simulator/components/grid"
	"github.com/Steelstone3/F1-Management-Simulator/components/names"
)

type Race struct {
	name names.RaceName
	grid grid.Grid
}

func constructRace() Race {
	return Race{}
}
