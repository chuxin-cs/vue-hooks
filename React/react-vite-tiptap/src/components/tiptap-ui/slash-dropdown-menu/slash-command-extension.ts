import { Extension } from "@tiptap/core"
import type { Editor } from "@tiptap/react"
import type { Transaction } from "@tiptap/pm/state"
import { Plugin, PluginKey } from "@tiptap/pm/state"

export type SlashCommandRange = { from: number; to: number }

export type SlashCommandItem = {
  id: string
  title: string
  keywords?: string[]
  command: (props: { editor: Editor; range: SlashCommandRange }) => void
}

export type SlashCommandPluginState = {
  active: boolean
  range: SlashCommandRange | null
  query: string
  selectedIndex: number
  items: SlashCommandItem[]
}

type SlashCommandMeta =
  | { type: "close" }
  | { type: "move"; direction: 1 | -1 }
  | { type: "setIndex"; index: number }

export const SlashCommandPluginKey = new PluginKey<SlashCommandPluginState>(
  "slashCommand"
)

export type SlashCommandOptions = {
  items?: (props: { editor: Editor; query: string }) => SlashCommandItem[]
}

function defaultItems(props: { editor: Editor; query: string }): SlashCommandItem[] {
  const { editor, query } = props
  const q = query.trim().toLowerCase()

  const all: SlashCommandItem[] = [
    {
      id: "heading-1",
      title: "Heading 1",
      keywords: ["h1", "title"],
      command: ({ editor }) => {
        editor.chain().focus().toggleHeading({ level: 1 }).run()
      },
    },
    {
      id: "heading-2",
      title: "Heading 2",
      keywords: ["h2"],
      command: ({ editor }) => {
        editor.chain().focus().toggleHeading({ level: 2 }).run()
      },
    },
    {
      id: "heading-3",
      title: "Heading 3",
      keywords: ["h3"],
      command: ({ editor }) => {
        editor.chain().focus().toggleHeading({ level: 3 }).run()
      },
    },
    {
      id: "heading-4",
      title: "Heading 4",
      keywords: ["h4"],
      command: ({ editor }) => {
        editor.chain().focus().toggleHeading({ level: 4 }).run()
      },
    },
    {
      id: "bullet-list",
      title: "Bullet List",
      keywords: ["ul", "list", "bullet"],
      command: ({ editor }) => {
        editor.chain().focus().toggleBulletList().run()
      },
    },
    {
      id: "ordered-list",
      title: "Numbered List",
      keywords: ["ol", "list", "numbered"],
      command: ({ editor }) => {
        editor.chain().focus().toggleOrderedList().run()
      },
    },
    {
      id: "task-list",
      title: "Task List",
      keywords: ["todo", "task", "check"],
      command: ({ editor }) => {
        editor.chain().focus().toggleList("taskList", "taskItem").run()
      },
    },
    {
      id: "blockquote",
      title: "Blockquote",
      keywords: ["quote"],
      command: ({ editor }) => {
        editor.chain().focus().toggleBlockquote().run()
      },
    },
    {
      id: "code-block",
      title: "Code Block",
      keywords: ["code"],
      command: ({ editor }) => {
        editor.chain().focus().toggleNode("codeBlock", "paragraph").run()
      },
    },
    {
      id: "horizontal-rule",
      title: "Divider",
      keywords: ["hr", "divider", "rule", "separator"],
      command: ({ editor }) => {
        editor.chain().focus().setHorizontalRule().run()
      },
    },
    {
      id: "image-upload",
      title: "Image",
      keywords: ["img", "image", "upload", "photo", "picture"],
      command: ({ editor }) => {
        editor.chain().focus().insertContent({ type: "imageUpload" }).run()
      },
    },
  ]

  const isAvailable = (item: SlashCommandItem) => {
    if (item.id === "image-upload") {
      return editor.extensionManager.extensions.some((ext) => ext.name === "imageUpload")
    }
    return true
  }

  const filtered = all
    .filter(isAvailable)
    .filter((item) => {
      if (!q) return true
      const haystack = [item.title, ...(item.keywords ?? [])]
        .join(" ")
        .toLowerCase()
      return haystack.includes(q)
    })

  return filtered
}

function getSlashMatch(props: { state: Editor["state"] }) {
  const { state } = props
  const { selection } = state
  if (!selection.empty) return null

  const $from = selection.$from
  const parentName = $from.parent.type.name
  if (parentName === "codeBlock") return null

  const textBefore = state.doc.textBetween($from.start(), selection.from, "\n", "\n")
  const match = /(?:^|\s)\/([^\s]*)$/.exec(textBefore)
  if (!match) return null

  const hasLeadingSpace = match[0].startsWith(" ")
  const from = selection.from - match[0].length + (hasLeadingSpace ? 1 : 0)
  const to = selection.from
  const query = match[1] ?? ""

  return { range: { from, to }, query }
}

export const SlashCommand = Extension.create<SlashCommandOptions>({
  name: "slashCommand",

  addOptions() {
    return {
      items: defaultItems,
    }
  },

  addProseMirrorPlugins() {
    const editor = this.editor

    return [
      new Plugin<SlashCommandPluginState>({
        key: SlashCommandPluginKey,
        state: {
          init: () => ({
            active: false,
            range: null,
            query: "",
            selectedIndex: 0,
            items: [],
          }),
          apply: (tr: Transaction, prev, _oldState, newState) => {
            const meta = tr.getMeta(SlashCommandPluginKey) as SlashCommandMeta | undefined
            if (meta?.type === "close") {
              return { ...prev, active: false, range: null, query: "", items: [], selectedIndex: 0 }
            }

            const match = getSlashMatch({ state: newState })
            if (!match) {
              return prev.active ? { ...prev, active: false, range: null, query: "", items: [], selectedIndex: 0 } : prev
            }

            const items = (this.options.items ?? defaultItems)({ editor, query: match.query })
            const queryChanged = !prev.active || prev.query !== match.query
            const nextSelectedBase = queryChanged ? 0 : prev.selectedIndex
            const maxIndex = Math.max(0, items.length - 1)
            let selectedIndex = Math.min(nextSelectedBase, maxIndex)

            if (meta?.type === "move" && items.length > 0) {
              selectedIndex = (selectedIndex + meta.direction + items.length) % items.length
            }

            if (meta?.type === "setIndex") {
              selectedIndex = Math.max(0, Math.min(meta.index, maxIndex))
            }

            return {
              active: true,
              range: match.range,
              query: match.query,
              selectedIndex,
              items,
            }
          },
        },
        props: {
          handleKeyDown: (view, event) => {
            const pluginState = SlashCommandPluginKey.getState(view.state)
            if (!pluginState?.active) return false

            if (event.key === "ArrowDown" || event.key === "ArrowUp") {
              event.preventDefault()
              view.dispatch(
                view.state.tr.setMeta(SlashCommandPluginKey, {
                  type: "move",
                  direction: event.key === "ArrowDown" ? 1 : -1,
                } satisfies SlashCommandMeta)
              )
              return true
            }

            if (event.key === "Escape") {
              event.preventDefault()
              view.dispatch(view.state.tr.setMeta(SlashCommandPluginKey, { type: "close" } satisfies SlashCommandMeta))
              return true
            }

            if (event.key === "Enter" || event.key === "Tab") {
              event.preventDefault()
              const item = pluginState.items[pluginState.selectedIndex]
              const range = pluginState.range
              if (!item || !range) return true

              editor.chain().focus().deleteRange(range).run()
              item.command({ editor, range })
              view.dispatch(view.state.tr.setMeta(SlashCommandPluginKey, { type: "close" } satisfies SlashCommandMeta))
              return true
            }

            return false
          },
        },
      }),
    ]
  },
})

