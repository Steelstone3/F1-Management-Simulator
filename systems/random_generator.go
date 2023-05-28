package systems

import (
	"math/rand"
	"time"
)

func GenerateRandomInRange(seed int64, minimumRange uint, maximumRange uint) uint {
	source := rand.NewSource(seed)
	random := rand.New(source)

	return uint(random.Intn(int(maximumRange)-int(minimumRange)) + int(minimumRange))
}

func GetRandomString(seed int64, strings []string) string {
	source := rand.NewSource(seed)
	random := rand.New(source)

	randomIndex := random.Intn(len(strings))

	return strings[randomIndex]
}

func GenerateSeed() int64 {
	source := rand.NewSource(time.Now().UnixNano())
	random := rand.New(source)

	return random.Int63()
}
