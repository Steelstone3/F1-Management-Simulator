# Notes

Notes on development direction

- Move team calculating overall for driver to the driver struct

- RaceGrid "calculates_race_result"
  - Creates array that is ordered by highest driver overall score to lowest
  - Later add random variance
- Have it so that a RaceGrid "awards_race_points()"
  - For each team
  - Given a DriverName
  - Find the given driver
  - Award the points
