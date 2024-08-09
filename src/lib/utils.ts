import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// Take an amount of seconds and convert it into a 01:34 type string.
export function formatTime(seconds: number): string {
  // Calculate minutes and seconds
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds % 60;

  // Format minutes and seconds with leading zeros if needed
  const formattedMinutes = minutes.toString().padStart(2, '0');
  const formattedSeconds = remainingSeconds.toString().padStart(2, '0');

  // Return the formatted string
  return `${formattedMinutes}:${formattedSeconds}`;
}

// Used to make sure an input only takes numeric values.
export function validateNumericInput(val: string): string {
    return val.replace(/[^0-9.]/g, '').replace(/\.(?=.*\.)/g, '')
}

