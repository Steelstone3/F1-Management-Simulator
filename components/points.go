package components

type Points struct {
	position uint
	points   uint
}

func calculatePoints(position uint) Points {
	switch position {
	case 1:
		return Points{position: position, points: 25}
	case 2:
		return Points{position: position, points: 18}
	case 3:
		return Points{position: position, points: 15}
	case 4:
		return Points{position: position, points: 12}
	case 5:
		return Points{position: position, points: 10}
	case 6:
		return Points{position: position, points: 8}
	case 7:
		return Points{position: position, points: 6}
	case 8:
		return Points{position: position, points: 4}
	case 9:
		return Points{position: position, points: 2}
	case 10:
		return Points{position: position, points: 1}
	default:
		return Points{position: position, points: 0}
	}
}
