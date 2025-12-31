"use client"

import { useCallback } from "react"
import type { Editor } from "@tiptap/react"
import { useEditorState } from "@tiptap/react"

import { useTiptapEditor } from "@/hooks/use-tiptap-editor"
import {
  SlashCommandPluginKey,
  type SlashCommandPluginState,
} from "@/components/tiptap-ui/slash-dropdown-menu/slash-command-extension"

export type UseSlashDropdownMenuConfig = {
  editor?: Editor | null
}

export function useSlashDropdownMenu(config?: UseSlashDropdownMenuConfig) {
  const { editor: providedEditor } = config || {}
  const { editor } = useTiptapEditor(providedEditor)

  const pluginState = useEditorState({
    editor,
    selector: ({ editor }) => {
      if (!editor) return null
      return SlashCommandPluginKey.getState(editor.state) ?? null
    },
  }) as SlashCommandPluginState | null

  const setIndex = useCallback(
    (index: number) => {
      if (!editor) return
      editor.view.dispatch(
        editor.state.tr.setMeta(SlashCommandPluginKey, { type: "setIndex", index })
      )
    },
    [editor]
  )

  const runItem = useCallback(
    (index: number) => {
      if (!editor) return
      const state = SlashCommandPluginKey.getState(editor.state)
      if (!state?.active || !state.range) return
      const item = state.items[index]
      if (!item) return

      editor.chain().focus().deleteRange(state.range).run()
      item.command({ editor, range: state.range })
      editor.view.dispatch(
        editor.state.tr.setMeta(SlashCommandPluginKey, { type: "close" })
      )
    },
    [editor]
  )

  const close = useCallback(() => {
    if (!editor) return
    editor.view.dispatch(
      editor.state.tr.setMeta(SlashCommandPluginKey, { type: "close" })
    )
  }, [editor])

  return {
    editor,
    state: pluginState,
    setIndex,
    runItem,
    close,
  }
}

