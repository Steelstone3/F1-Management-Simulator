package entities

import "github.com/Steelstone3/F1-Management-Simulator/components/names"

type Race struct {
	name names.RaceName
}

func constructRace() Race {
	return Race{}
}
