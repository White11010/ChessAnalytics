type FormatOptions = {
  withRelativeDays?: boolean;
};

export function formatTimestamp(
  timestamp: number | string | Date,
  options: FormatOptions = {},
): string {
  const { withRelativeDays = false } = options;

  const date = new Date(timestamp);
  const now = new Date();

  const isSameDay = (d1: Date, d2: Date) =>
    d1.getFullYear() === d2.getFullYear() &&
    d1.getMonth() === d2.getMonth() &&
    d1.getDate() === d2.getDate();

  const yesterday = new Date(now);
  yesterday.setDate(now.getDate() - 1);

  let hours = date.getHours();
  const minutes = date.getMinutes().toString().padStart(2, '0');
  const ampm = hours >= 12 ? 'pm' : 'am';
  hours = hours % 12 || 12;

  const time = `${hours}:${minutes}${ampm}`;

  if (withRelativeDays) {
    if (isSameDay(date, now)) return time; // Today → только время
    if (isSameDay(date, yesterday)) return `Yesterday ${time}`;
  }

  const months = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec',
  ];

  const day = date.getDate();
  const month = months[date.getMonth()];

  return `${day} ${month} ${time}`;
}
