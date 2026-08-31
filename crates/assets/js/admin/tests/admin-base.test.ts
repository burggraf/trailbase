import { describe, expect, test } from "vitest";
import { adminBasePath } from "@/lib/admin-base";

describe("adminBasePath", () => {
  test.each([
    ["/_/admin-refresh/", "/_/admin-refresh"],
    ["/", ""],
  ])("normalizes %s", (base, expected) => {
    expect(adminBasePath(base)).toBe(expected);
  });
});
