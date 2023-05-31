package grid

type Points struct {
	points   uint
}

func calculatePoints(position Position) Points {
	switch position.position {
	case 1:
		return Points{points: 25}
	case 2:
		return Points{points: 18}
	case 3:
		return Points{points: 15}
	case 4:
		return Points{points: 12}
	case 5:
		return Points{points: 10}
	case 6:
		return Points{points: 8}
	case 7:
		return Points{points: 6}
	case 8:
		return Points{points: 4}
	case 9:
		return Points{points: 2}
	case 10:
		return Points{points: 1}
	default:
		return Points{points: 0}
	}
}
