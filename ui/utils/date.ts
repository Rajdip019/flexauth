import { fromUnixTime, addDays, format } from 'date-fns';

// Define a function to convert timestamp and format date with added days
export function formatTimestampWithAddedDays(timestamp: number, daysToAdd: number): string {
    // Convert the timestamp (in seconds) to a Date object
    const date = fromUnixTime(timestamp / 1000);

    // Add the specified number of days
    const newDate = addDays(date, daysToAdd);

    // Format the date as "DD/MM/YYYY, HH:mm:ss"
    const formattedDate = format(newDate, "dd/MM/yyyy, HH:mm:ss");

    return formattedDate;
}
