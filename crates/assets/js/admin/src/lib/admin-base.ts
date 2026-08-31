export function adminBasePath(base = import.meta.env.BASE_URL): string {
  return base === "/" ? "" : base.replace(/\/$/, "");
}

export function adminPath(base: string, route: string): string {
  return `${adminBasePath(base)}/${route.replace(/^\/+/, "")}`;
}
