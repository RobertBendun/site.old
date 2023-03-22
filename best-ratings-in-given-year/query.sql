.import --csv ./ratings.csv ratings

SELECT "Title", "Your Rating", SUBSTR("Release Date",0,5)
FROM ratings AS r1
WHERE "Title Type" IN ('movie', 'tvSeries', 'tvMiniSeries', 'tvMovie')
AND CAST("Your Rating" AS INT) = (
	SELECT MAX(CAST("Your Rating" AS INT))
	FROM ratings as r2
	WHERE SUBSTR(r1."Release Date", 0, 5) == SUBSTR(r2."Release Date", 0, 5)
)
ORDER BY "Release Date";
