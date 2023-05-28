package car

type CarStatistics struct {
	aero           uint
	engine         uint
	reliability    uint
	tireManagement uint
}

func constructCarStatistics() CarStatistics {
	return CarStatistics{}
}

func calculateOverallCarStatistics() uint {
	return 0
}
