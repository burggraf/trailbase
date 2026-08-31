import { describe, expect, test } from "vitest";
import { adminBasePath, adminPath } from "@/lib/admin-base";

describe("adminBasePath", () => {
  test.each([
    ["/_/admin-refresh/", "/_/admin-refresh"],
    ["/", ""],
  ])("normalizes %s", (base, expected) => {
    expect(adminBasePath(base)).toBe(expected);
  });

  test("joins base and route with one slash", () => {
    expect(adminPath("/table/", "/_/admin-refresh/")).toBe(
      "/_/admin-refresh/table/",
    );
    expect(adminPath("/table", "/")).toBe("/table");
    expect(adminPath("/", "/_/admin-refresh/")).toBe("/_/admin-refresh/");
    expect(adminPath("/", "/")).toBe("/");
  });
});
