export const timeArrayToString = (timeArray: number[]): string => {
  const [year, month, day, hour, minute, nanosecond, offsetHour, offsetMinute, second] = timeArray;

  return `${year}-${month.toString().padStart(2, '0')}-${day.toString().padStart(2, '0')} ${hour.toString().padStart(2, '0')}:${minute.toString().padStart(2, '0')}:${second.toString().padStart(2, '0')}.${nanosecond.toString().padStart(3, '0')}+${offsetHour.toString().padStart(2, '0')}:${offsetMinute.toString().padStart(2, '0')}`;
}
