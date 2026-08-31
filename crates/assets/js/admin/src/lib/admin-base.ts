export function adminBasePath(base = import.meta.env.BASE_URL): string {
  return base === "/" ? "" : base.replace(/\/$/, "");
}

export function adminPath(
  route: string,
  base = import.meta.env.BASE_URL,
): string {
  return `${adminBasePath(base)}/${route.replace(/^\/+/, "")}`;
}
