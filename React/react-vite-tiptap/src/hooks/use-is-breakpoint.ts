import { useEffect, useState } from "react"

type BreakpointMode = "min" | "max"

/**
 * Hook to detect whether the current viewport matches a given breakpoint rule.
 * Example:
 *   useIsBreakpoint("max", 768)   // true when width < 768
 *   useIsBreakpoint("min", 1024)  // true when width >= 1024
 */
export function useIsBreakpoint(
  mode: BreakpointMode = "max",
  breakpoint = 768
) {
  const [matches, setMatches] = useState<boolean | undefined>(() => {
    if (typeof window === "undefined") return undefined

    const query =
      mode === "min"
        ? `(min-width: ${breakpoint}px)`
        : `(max-width: ${breakpoint - 1}px)`

    return window.matchMedia(query).matches
  })

  useEffect(() => {
    if (typeof window === "undefined") return

    const query =
      mode === "min"
        ? `(min-width: ${breakpoint}px)`
        : `(max-width: ${breakpoint - 1}px)`

    const mql = window.matchMedia(query)
    const onChange = (e: MediaQueryListEvent) => setMatches(e.matches)

    queueMicrotask(() => setMatches(mql.matches))

    // Add listener
    mql.addEventListener("change", onChange)
    return () => mql.removeEventListener("change", onChange)
  }, [mode, breakpoint])

  return !!matches
}
