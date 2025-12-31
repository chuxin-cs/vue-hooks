"use client"

import { useCallback, useRef } from "react"

// basically Exclude<React.ClassAttributes<T>["ref"], string>
type UserRef<T> =
  | ((instance: T | null) => void)
  | React.RefObject<T | null>
  | null
  | undefined

const updateRef = <T>(
  targetRef: NonNullable<UserRef<T>>,
  value: T | null
) => {
  if (typeof targetRef === "function") {
    targetRef(value)
  } else if (
    targetRef &&
    typeof targetRef === "object" &&
    "current" in targetRef
  ) {
    // Safe assignment without MutableRefObject
    ;(targetRef as { current: T | null }).current = value
  }
}

export const useComposedRef = <T extends HTMLElement>(
  libRefRef: React.RefObject<T | null>,
  userRef: UserRef<T>
) => {
  const prevUserRef = useRef<UserRef<T>>(null)

  return useCallback(
    (instance: T | null) => {
      if (libRefRef && "current" in libRefRef) {
        ;(libRefRef as { current: T | null }).current = instance
      }

      if (prevUserRef.current) {
        updateRef(prevUserRef.current, null)
      }

      prevUserRef.current = userRef

      if (userRef) {
        updateRef(userRef, instance)
      }
    },
    [libRefRef, userRef]
  )
}

export default useComposedRef
