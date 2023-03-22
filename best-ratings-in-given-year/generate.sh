#!/bin/sh

sqlite3 <./query.sql > data.table
cut -d'|' -f3 data.table | uniq -c > data.years

cat header.html

last_year=""
while read -r line; do
	year="$(echo "$line" | cut -d'|' -f3)"
	if [ "$last_year" != "$year" ]; then
		rowspan="$(awk '/'"$year"'/ { print $1 }' data.years)"
		year_html='<td rowspan=\"'"$rowspan"'\">" $3 "</td>'
		score_html='<td rowspan=\"'$rowspan'\">" $2 "</td>'
	else
		year_html=''
		score_html=''
	fi
	echo "$line" | awk -F'|' '{ print "<tr>'"$year_html"'<td data-title=\"" $1 "\">" $1 "</td>'"$score_html"'</tr>" }'
	last_year="$year"
done < data.table

cat footer.html
