export function adminBasePath(base = import.meta.env.BASE_URL): string {
  return base === "/" ? "" : base.replace(/\/$/, "");
}
