package driver

type DriverStatistics struct {
	experience uint
	raceCraft  uint
	awareness  uint
	pace       uint
}

func constructDriverStatistics() DriverStatistics {
	return DriverStatistics{}
}

func (d *DriverStatistics) calculateOverallDriverStatistics() uint {
	return 0
}

func (d *DriverStatistics) calculateRaceChance() float32 {
	return 0.0
}
