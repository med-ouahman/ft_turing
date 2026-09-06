BASE_ERROR_LOG="var/log/error_log.log"

ERROR_LOG="$BASE_ERROR_LOG"
i=0

while [ -f "$ERROR_LOG" ]
do
    i=$((i + 1))
    ERROR_LOG="${BASE_ERROR_LOG}_${i}"
done

cargo run 2> /dev/null
