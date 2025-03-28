export function toLocaleDate(input: string): string {
  const ts = Date.parse(input);
  const date = new Date(ts);
  return date.toLocaleDateString("ru-RU", {
    day: "numeric",
    month: "long",
    year: "numeric",
  })
}

export function toLocale(input: string): string {
  const ts = Date.parse(input);
  const date = new Date(ts);
  return date.toLocaleDateString("ru-RU", {
    day: "numeric",
    month: "long",
    year: "numeric",
    hour: "numeric",
    minute: "numeric",
  })
}
