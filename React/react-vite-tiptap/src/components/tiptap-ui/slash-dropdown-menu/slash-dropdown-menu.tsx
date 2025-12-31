"use client"

import { useEffect, useMemo } from "react"
import { createPortal } from "react-dom"
import { autoUpdate, flip, offset, shift, useFloating } from "@floating-ui/react"

import {
  useSlashDropdownMenu,
  type UseSlashDropdownMenuConfig,
} from "@/components/tiptap-ui/slash-dropdown-menu/use-slash-dropdown-menu"

import { Card, CardBody } from "@/components/tiptap-ui-primitive/card"
import { Button } from "@/components/tiptap-ui-primitive/button"

import { HeadingOneIcon } from "@/components/tiptap-icons/heading-one-icon"
import { HeadingTwoIcon } from "@/components/tiptap-icons/heading-two-icon"
import { HeadingThreeIcon } from "@/components/tiptap-icons/heading-three-icon"
import { HeadingFourIcon } from "@/components/tiptap-icons/heading-four-icon"
import { ListIcon } from "@/components/tiptap-icons/list-icon"
import { ListOrderedIcon } from "@/components/tiptap-icons/list-ordered-icon"
import { ListTodoIcon } from "@/components/tiptap-icons/list-todo-icon"
import { BlockquoteIcon } from "@/components/tiptap-icons/blockquote-icon"
import { CodeBlockIcon } from "@/components/tiptap-icons/code-block-icon"
import { ImagePlusIcon } from "@/components/tiptap-icons/image-plus-icon"

import "@/components/tiptap-ui/slash-dropdown-menu/slash-dropdown-menu.scss"

export type SlashDropdownMenuProps = UseSlashDropdownMenuConfig & {
  emptyText?: string
}

const iconMap: Record<string, React.FC<{ className?: string }>> = {
  "heading-1": HeadingOneIcon,
  "heading-2": HeadingTwoIcon,
  "heading-3": HeadingThreeIcon,
  "heading-4": HeadingFourIcon,
  "bullet-list": ListIcon,
  "ordered-list": ListOrderedIcon,
  "task-list": ListTodoIcon,
  blockquote: BlockquoteIcon,
  "code-block": CodeBlockIcon,
  "image-upload": ImagePlusIcon,
}

export function SlashDropdownMenu(props: SlashDropdownMenuProps) {
  const { emptyText = "No results", ...config } = props
  const { editor, state, setIndex, runItem } = useSlashDropdownMenu(config)

  const rangeFrom = state?.range?.from ?? null

  const virtualReference = useMemo(() => {
    if (!editor || rangeFrom === null) return null
    return {
      getBoundingClientRect: () => {
        const coords = editor.view.coordsAtPos(rangeFrom)
        return {
          x: coords.left,
          y: coords.bottom,
          width: 0,
          height: 0,
          top: coords.bottom,
          left: coords.left,
          right: coords.left,
          bottom: coords.bottom,
          toJSON: () => "",
        } as DOMRect
      },
    }
  }, [editor, rangeFrom])

  const { refs, floatingStyles, update } = useFloating({
    placement: "bottom-start",
    middleware: [offset(8), flip(), shift({ padding: 8 })],
    whileElementsMounted: autoUpdate,
  })

  useEffect(() => {
    if (virtualReference) {
      refs.setReference(virtualReference as any)
    }
  }, [refs, virtualReference])

  useEffect(() => {
    update()
  }, [update, state?.active, state?.query, state?.selectedIndex, state?.items?.length])

  if (!editor || !state?.active) return null

  return createPortal(
    <div
      ref={refs.setFloating}
      style={floatingStyles}
      className="tiptap-slash-dropdown-menu"
    >
      <Card>
        <CardBody className="tiptap-slash-dropdown-menu-body">
          {state.items.length === 0 ? (
            <div className="tiptap-slash-dropdown-menu-empty">{emptyText}</div>
          ) : (
            state.items.map((item, index) => {
              const Icon = iconMap[item.id]
              const selected = index === state.selectedIndex

              return (
                <Button
                  key={item.id}
                  type="button"
                  data-style="ghost"
                  data-highlighted={selected ? "true" : "false"}
                  className="tiptap-slash-dropdown-menu-item"
                  showTooltip={false}
                  onMouseEnter={() => setIndex(index)}
                  onMouseDown={(e) => e.preventDefault()}
                  onClick={() => runItem(index)}
                >
                  {Icon ? <Icon className="tiptap-button-icon" /> : null}
                  <span className="tiptap-slash-dropdown-menu-item-title">
                    {item.title}
                  </span>
                </Button>
              )
            })
          )}
        </CardBody>
      </Card>
    </div>,
    document.body
  )
}
