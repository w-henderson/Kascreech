export default function getSuffix(num: number): string {
  const MOD_10_SUFFIXES = ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"];
  const MOD_100_EXCEPTIONS: any = {
    11: "th",
    12: "th",
    13: "th"
  }

  let mod100 = num % 100;
  let mod10 = num % 10;

  if (MOD_100_EXCEPTIONS[mod100]) {
    return MOD_100_EXCEPTIONS[mod100]
  } else {
    return MOD_10_SUFFIXES[mod10]
  }
}